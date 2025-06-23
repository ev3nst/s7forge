use futures_util::FutureExt;
use steamworks::{AppIDs, AppId, UGCQueryType, UGCType};
use tokio::sync::mpsc;

use crate::commands::fetch_creator_names::fetch_creator_names;
use crate::commands::get_workshop_items::EnhancedWorkshopItem;
use crate::core::steam_manager;
use crate::core::workshop_item::workshop::{WorkshopItem, WorkshopItemsResult};

pub async fn search_workshop(
    steam_game_id: u32,
    search_text: String,
    max_results: u32,
) -> Result<Vec<EnhancedWorkshopItem>, String> {
    let steam_client = steam_manager::initialize_client(steam_game_id).await?;

    let (tx, mut rx) = mpsc::channel(32);

    let search_task = tokio::task::spawn_blocking(move || {
        let ugc = steam_client.ugc();
        let (tx_inner, rx_inner) = std::sync::mpsc::channel();
        let app_ids = AppIDs::Both {
            creator: AppId(steam_game_id),
            consumer: AppId(steam_game_id),
        };
        let query_handle = ugc
            .query_all(UGCQueryType::RankedByTextSearch, UGCType::Items, app_ids, 1)
            .map_err(|e| format!("Failed to create search query: {}", e))?;
        query_handle
            .set_search_text(&search_text)
            .set_ranked_by_trend_days(7)
            .set_return_metadata(true)
            .set_return_children(true)
            .set_return_additional_previews(true)
            .set_return_key_value_tags(true)
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
                return Err("Search operation timed out waiting for Steam response".to_string());
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    let mut search_result = None;
    let mut fused_task = search_task.fuse();

    while search_result.is_none() {
        tokio::select! {
            Some(_) = rx.recv() => {
                steam_manager::run_callbacks(steam_game_id)?;
            }
            task_result = &mut fused_task => {
                search_result = Some(task_result.map_err(|e| format!("Task error: {}", e))??);
                break;
            }
        }
    }

    let items_result = search_result.unwrap();
    let workshop_items = items_result
        .items
        .into_iter()
        .filter_map(|item| match item {
            Some(it) if it.file_type == "Community" => Some(it),
            _ => None,
        })
        .take(max_results as usize)
        .collect::<Vec<WorkshopItem>>();

    if workshop_items.is_empty() {
        return Ok(Vec::new());
    }

    // Fetch creator names
    let creator_ids: Vec<steamworks::SteamId> = workshop_items
        .iter()
        .map(|item| steamworks::SteamId::from_raw(item.owner.steam_id64))
        .collect();

    let creator_names = fetch_creator_names(creator_ids, steam_game_id).await?;

    let result = workshop_items
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
