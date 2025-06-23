use steamworks::PublishedFileId;
use tokio::task;

use crate::commands::workshop_items::{EnhancedWorkshopItem, workshop_items};
use crate::core::steam_manager;

pub async fn subscribed_items(steam_game_id: u32) -> Result<Vec<EnhancedWorkshopItem>, String> {
    let steam_client = steam_manager::initialize_client(steam_game_id).await?;

    let subscribed_items: Vec<PublishedFileId> = task::spawn_blocking({
        let steam_client = steam_client.clone();
        move || steam_client.ugc().subscribed_items()
    })
    .await
    .map_err(|e| format!("Failed to fetch subscribed items: {:?}", e))?;

    let item_ids: Vec<u64> = subscribed_items.iter().map(|id| id.0).collect();
    if item_ids.is_empty() {
        return Ok(Vec::new());
    }

    workshop_items(steam_game_id, item_ids).await
}
