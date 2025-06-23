use futures_util::FutureExt;
use steamworks::PublishedFileId;
use tokio::sync::mpsc;

use crate::core::steam_manager;

pub async fn unsubscribe(steam_game_id: u32, item_id: u64) -> Result<bool, String> {
    let steam_client = steam_manager::initialize_client(steam_game_id).await?;

    let (tx, mut rx) = mpsc::channel(32);

    let unsub_task = tokio::task::spawn_blocking(move || {
        let ugc = steam_client.ugc();
        let (tx_inner, rx_inner) = std::sync::mpsc::channel();

        ugc.unsubscribe_item(PublishedFileId(item_id), move |result| {
            let _ = tx_inner.send(result);
        });

        let start_time = std::time::Instant::now();
        let timeout_duration = std::time::Duration::from_secs(30);

        loop {
            let _ = tx.blocking_send(());
            if let Ok(result) = rx_inner.try_recv() {
                return result
                    .map(|_| true)
                    .map_err(|e| format!("Steam API error: {}", e));
            }

            if start_time.elapsed() > timeout_duration {
                return Err("Operation timed out waiting for Steam response".to_string());
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    let mut result = None;
    let mut unsub_task = unsub_task.fuse();

    while result.is_none() {
        tokio::select! {
            Some(_) = rx.recv() => {
                steam_manager::run_callbacks(steam_game_id)?;
            }
            task_result = &mut unsub_task => {
                result = Some(task_result.unwrap_or_else(|e| Err(format!("Task join error: {}", e)))?);
                break;
            }
        }
    }

    Ok(result.unwrap())
}
