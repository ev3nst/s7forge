use steamworks::PublishedFileId;
use tokio::task;

use crate::commands::get_workshop_items::get_workshop_items;
use crate::core::steam_manager;
use crate::core::workshop_item::workshop::WorkshopItem;

pub async fn get_subscribed_items(steam_game_id: u32) -> Result<Vec<WorkshopItem>, String> {
    let steam_client = steam_manager::initialize_client(steam_game_id).await?;

    let subscribed_items: Vec<PublishedFileId> = task::spawn_blocking({
        let steam_client = steam_client.clone();
        move || steam_client.ugc().subscribed_items()
    })
    .await
    .map_err(|e| format!("Failed to fetch subscribed items: {}", e))?;

    let item_ids: Vec<u64> = subscribed_items.iter().map(|id| id.0).collect();
    if item_ids.is_empty() {
        return Ok(Vec::new());
    }

    get_workshop_items(steam_game_id, item_ids).await
}
