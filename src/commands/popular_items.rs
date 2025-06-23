use futures_util::FutureExt;
use steamworks::{AppIDs, AppId, UGCQueryType, UGCType};
use tokio::sync::mpsc;

use crate::commands::workshop_items::EnhancedWorkshopItem;
use crate::core::steam_manager;
use crate::core::workshop_item::workshop::{WorkshopItem, WorkshopItemsResult};
use crate::utils::fetch_creator_names::fetch_creator_names;

pub async fn popular_items(
    steam_game_id: u32,
    period: String,
    page: u32,
) -> Result<Vec<EnhancedWorkshopItem>, String> {
    if page == 0 {
        return Err("Page number must be at least 1".to_string());
    }

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
            .query_all(UGCQueryType::RankedByVote, UGCType::Items, app_ids, page)
            .map_err(|e| format!("Failed to create popular items query: {:?}", e))?;

        let configured_query = match period.as_str() {
            "today" | "one-week" | "three-months" | "six-months" | "one-year" => {
                let trend_days = match period.as_str() {
                    "today" => 1,
                    "one-week" => 7,
                    "three-months" => 90,
                    "six-months" => 180,
                    "one-year" => 365,
                    _ => 7,
                };

                ugc.query_all(UGCQueryType::RankedByTrend, UGCType::Items, app_ids, 1)
                    .map_err(|e| format!("Failed to create trend query: {:?}", e))?
                    .set_ranked_by_trend_days(trend_days)
            }
            _ => query_handle,
        };

        configured_query
            .set_return_metadata(true)
            .set_return_children(true)
            .set_return_additional_previews(true)
            .set_return_key_value_tags(true)
            .fetch(move |fetch_result| {
                let _ = tx_inner.send(
                    fetch_result
                        .map(|query_results| WorkshopItemsResult::from_query_results(query_results))
                        .map_err(|e| format!("Steam API error: {:?}", e)),
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
                return Err(
                    "Popular items operation timed out waiting for Steam response".to_string(),
                );
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
        .collect::<Vec<WorkshopItem>>();

    if workshop_items.is_empty() {
        return Ok(Vec::new());
    }

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
