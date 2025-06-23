mod commands;
mod core;
mod utils;

use clap::{Parser, Subcommand};
use steamworks::SteamId;

#[derive(Parser)]
#[command(name = "s7forge")]
#[command(about = "Steamworks utility wrapper")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CheckItemDownload {
        #[arg(long)]
        app_id: u32,
        #[arg(long)]
        item_id: u64,
    },
    FetchCreatorNames {
        #[arg(long)]
        app_id: u32,
        #[arg(long, value_delimiter = ',')]
        creator_ids: Vec<u64>,
    },
    GetCollectionItems {
        #[arg(long)]
        app_id: u32,
        #[arg(long)]
        item_id: u64,
    },
    GetWorkshopItems {
        #[arg(long)]
        app_id: u32,
        #[arg(long, value_delimiter = ',')]
        item_ids: Vec<u64>,
    },
    Subscribe {
        #[arg(long)]
        app_id: u32,
        #[arg(long)]
        item_id: u64,
    },
    Unsubscribe {
        #[arg(long)]
        app_id: u32,
        #[arg(long)]
        item_id: u64,
    },
    UpdateWorkshopItem {
        #[arg(long)]
        app_id: u32,
        #[arg(long)]
        item_id: u64,
    },
    GetSubscribedItems {
        #[arg(long)]
        app_id: u32,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::CheckItemDownload { app_id, item_id } => {
            commands::check_item_download::check_item_download(app_id, item_id)
                .await
                .map(|info| serde_json::to_string_pretty(&info).unwrap())
        }
        Commands::FetchCreatorNames {
            app_id,
            creator_ids,
        } => {
            let steam_ids: Vec<SteamId> = creator_ids.into_iter().map(SteamId::from_raw).collect();
            commands::fetch_creator_names::fetch_creator_names(steam_ids, app_id)
                .await
                .map(|names| serde_json::to_string_pretty(&names).unwrap())
        }
        Commands::GetCollectionItems { app_id, item_id } => {
            commands::get_collection_items::get_collection_items(app_id, item_id)
                .await
                .map(|items| serde_json::to_string_pretty(&items).unwrap())
        }
        Commands::GetWorkshopItems { app_id, item_ids } => {
            commands::get_workshop_items::get_workshop_items(app_id, item_ids)
                .await
                .map(|items| serde_json::to_string_pretty(&items).unwrap())
        }
        Commands::Subscribe { app_id, item_id } => commands::subscribe::subscribe(app_id, item_id)
            .await
            .map(|success| serde_json::to_string_pretty(&success).unwrap()),
        Commands::Unsubscribe { app_id, item_id } => {
            commands::unsubscribe::unsubscribe(app_id, item_id)
                .await
                .map(|success| serde_json::to_string_pretty(&success).unwrap())
        }
        Commands::UpdateWorkshopItem { app_id, item_id } => {
            commands::update_workshop_item::update_workshop_item(app_id, item_id)
                .await
                .map(|_| "\"Workshop item update completed successfully\"".to_string())
        }
        Commands::GetSubscribedItems { app_id } => {
            commands::subscribed_items::get_subscribed_items(app_id)
                .await
                .map(|items| serde_json::to_string_pretty(&items).unwrap())
        }
    };

    match result {
        Ok(output) => {
            println!("{}", output);
            std::process::exit(0);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
