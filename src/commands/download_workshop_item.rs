use std::sync::{Arc, Mutex};
use std::time::Duration;
use steamworks::{ItemState, PublishedFileId};

use crate::core::steam_manager;

pub async fn download_workshop_item(steam_game_id: u32, item_id: u64) -> Result<(), String> {
    let steam_client = steam_manager::initialize_client(steam_game_id).await?;

    let published_file_id = PublishedFileId(item_id);
    {
        let ugc = steam_client.ugc();
        let state = ugc.item_state(published_file_id);
        if !state.contains(ItemState::SUBSCRIBED) {
            return Err("Workshop item is not subscribed".to_string());
        }

        ugc.download_item(published_file_id, true);
    }

    let timeout = Duration::from_secs(10 * 60); // 10 minutes
    let cancelled = Arc::new(Mutex::new(false));
    let cancelled_clone = cancelled.clone();

    let (tx, rx) = tokio::sync::oneshot::channel();
    std::thread::spawn(move || {
        let start_time = std::time::Instant::now();
        loop {
            if *cancelled_clone.lock().unwrap() {
                let _ = tx.send(Err("Download cancelled".to_string()));
                break;
            }

            if start_time.elapsed() > timeout {
                let _ = tx.send(Err(format!(
                    "Download timeout after {} minutes",
                    timeout.as_secs() / 60
                )));
                break;
            }

            let ugc = steam_client.ugc();
            let state = ugc.item_state(published_file_id);

            if let Some((downloaded, total)) = ugc.item_download_info(published_file_id) {
                if downloaded == total && total > 0 {
                    let _ = tx.send(Ok(()));
                    break;
                }
            }

            if state.contains(ItemState::INSTALLED)
                && state.contains(ItemState::SUBSCRIBED)
                && !state.contains(ItemState::DOWNLOADING)
                && !state.contains(ItemState::DOWNLOAD_PENDING)
            {
                let _ = tx.send(Ok(()));
                break;
            }

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    match rx.await {
        Ok(result) => result,
        Err(_) => {
            *cancelled.lock().unwrap() = true;
            Err("Download monitoring failed unexpectedly".to_string())
        }
    }
}
