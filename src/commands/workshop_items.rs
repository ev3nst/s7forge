use bincode::{Decode, Encode};
use std::fs;

use futures_util::FutureExt;
use rustc_hash::FxHashMap;
use serde::Serialize;
use steamworks::{PublishedFileId, SteamId};

use crate::commands::fetch_creator_names::fetch_creator_names;
use crate::core::steam_manager;
use crate::core::workshop_item::workshop::{WorkshopItem, WorkshopItemsResult};
use crate::utils::get_cache_dir::get_cache_dir;

#[derive(Debug, Encode, Decode)]
pub struct WorkshopItemCache {
    pub items: FxHashMap<u64, WorkshopItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnhancedWorkshopItem {
    #[serde(flatten)]
    pub workshop_item: WorkshopItem,
    pub creator_name: String,
}

impl EnhancedWorkshopItem {
    pub fn new(workshop_item: WorkshopItem, creator_name: String) -> Self {
        Self {
            workshop_item,
            creator_name,
        }
    }
}

pub async fn workshop_items(
    steam_game_id: u32,
    item_ids: Vec<u64>,
) -> Result<Vec<EnhancedWorkshopItem>, String> {
    if item_ids.is_empty() {
        return Ok(Vec::new());
    }

    let cache_dir = get_cache_dir()?;
    fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("Failed to create cache directory: {}", e))?;

    let cache_path = cache_dir.join("workshop_items_cache.bin");
    let bincode_config = bincode::config::standard();

    let mut cached_items: FxHashMap<u64, WorkshopItem> = FxHashMap::default();
    if cache_path.exists() {
        if let Ok(cache_content) = fs::read(&cache_path) {
            if let Ok((cache_entry, _)) =
                bincode::decode_from_slice::<WorkshopItemCache, _>(&cache_content, bincode_config)
            {
                cached_items = cache_entry.items;
            }
        }
    }

    let ids_to_fetch: Vec<u64> = item_ids
        .iter()
        .filter(|id| !cached_items.contains_key(id))
        .cloned()
        .collect();
    if ids_to_fetch.is_empty() {
        let workshop_items: Vec<WorkshopItem> = item_ids
            .iter()
            .filter_map(|id| cached_items.get(id).cloned())
            .collect();
        let creator_ids: Vec<SteamId> = workshop_items
            .iter()
            .map(|item| SteamId::from_raw(item.owner.steam_id64))
            .collect();

        let creator_names = fetch_creator_names(creator_ids, steam_game_id).await?;

        return Ok(workshop_items
            .into_iter()
            .map(|item| {
                let creator_name = creator_names
                    .get(&item.owner.steam_id64)
                    .cloned()
                    .unwrap_or_else(|| "[unknown]".to_string());
                EnhancedWorkshopItem::new(item, creator_name)
            })
            .collect());
    }

    let steam_client = steam_manager::initialize_client(steam_game_id).await?;

    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let items_task = tokio::task::spawn_blocking(move || {
        let ugc = steam_client.ugc();
        let (tx_inner, rx_inner) = std::sync::mpsc::channel();
        let query_handle = ugc
            .query_items(ids_to_fetch.iter().map(|id| PublishedFileId(*id)).collect())
            .map_err(|e| format!("Failed to create query handle: {}", e))?;

        query_handle
            .include_children(true)
            .fetch(move |fetch_result| {
                let _ = tx_inner.send(
                    fetch_result
                        .map(|query_results| WorkshopItemsResult::from_query_results(query_results))
                        .map_err(|e| format!("Steam API error: {}", e)),
                );
            });

        let start_time = std::time::Instant::now();
        let timeout_duration = std::time::Duration::from_secs(30);

        loop {
            let _ = tx.blocking_send(());
            if let Ok(result) = rx_inner.try_recv() {
                return result;
            }

            if start_time.elapsed() > timeout_duration {
                return Err("Operation timed out waiting for Steam response".to_string());
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    let mut items_result = None;
    let mut fused_task = items_task.fuse();

    while items_result.is_none() {
        tokio::select! {
            Some(_) = rx.recv() => {
                steam_manager::run_callbacks(steam_game_id)?;
            }
            task_result = &mut fused_task => {
                items_result = Some(
                    task_result.map_err(|e| format!("Task error: {}", e))?
                );
                break;
            }
        }
    }

    let items_result = items_result.unwrap()?;
    let fetched_items = items_result
        .items
        .into_iter()
        .filter_map(|item| match item {
            Some(it) if it.file_type == "Community" => Some(it),
            _ => None,
        })
        .collect::<Vec<WorkshopItem>>();

    for item in &fetched_items {
        cached_items.insert(item.published_file_id, item.clone());
    }
    let cache_struct = WorkshopItemCache {
        items: cached_items.clone(),
    };
    let serialized_cache = bincode::encode_to_vec(&cache_struct, bincode_config)
        .map_err(|e| format!("Failed to serialize cache: {}", e))?;
    let _ = fs::write(&cache_path, serialized_cache);

    let final_items: Vec<WorkshopItem> = item_ids
        .iter()
        .filter_map(|id| cached_items.get(id).cloned())
        .collect();

    let creator_ids: Vec<SteamId> = final_items
        .iter()
        .map(|item| SteamId::from_raw(item.owner.steam_id64))
        .collect();

    let creator_names = fetch_creator_names(creator_ids, steam_game_id).await?;

    let result = final_items
        .into_iter()
        .map(|item| {
            let creator_name = creator_names
                .get(&item.owner.steam_id64)
                .cloned()
                .unwrap_or_else(|| "[unknown]".to_string());
            EnhancedWorkshopItem::new(item, creator_name)
        })
        .collect();

    Ok(result)
}
