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
    CollectionItems {
        #[arg(long)]
        app_id: u32,
        #[arg(long)]
        item_id: u64,
    },
    WorkshopItems {
        #[arg(long)]
        app_id: u32,
        #[arg(long, value_delimiter = ',')]
        item_ids: Vec<u64>,
    },
    Subscribe {
        #[arg(long)]
        app_id: u32,
        #[arg(long, value_delimiter = ',')]
        item_ids: Vec<u64>,
    },
    Unsubscribe {
        #[arg(long)]
        app_id: u32,
        #[arg(long, value_delimiter = ',')]
        item_ids: Vec<u64>,
    },
    UpdateWorkshopItem {
        #[arg(long)]
        app_id: u32,
        #[arg(long)]
        item_id: u64,
    },
    SubscribedItems {
        #[arg(long)]
        app_id: u32,
    },
    SearchWorkshop {
        #[arg(long)]
        app_id: u32,
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "10")]
        max_results: u32,
    },
    WorkshopPath {
        #[arg(long)]
        app_id: u32,
    },
    SteamLibraryPaths,
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
        Commands::CollectionItems { app_id, item_id } => {
            commands::collection_items::collection_items(app_id, item_id)
                .await
                .map(|items| serde_json::to_string_pretty(&items).unwrap())
        }
        Commands::WorkshopItems { app_id, item_ids } => {
            commands::workshop_items::workshop_items(app_id, item_ids)
                .await
                .map(|items| serde_json::to_string_pretty(&items).unwrap())
        }
        Commands::Subscribe { app_id, item_ids } => {
            commands::subscribe::subscribe(app_id, item_ids)
                .await
                .map(|results| serde_json::to_string_pretty(&results).unwrap())
        }
        Commands::Unsubscribe { app_id, item_ids } => {
            commands::unsubscribe::unsubscribe(app_id, item_ids)
                .await
                .map(|results| serde_json::to_string_pretty(&results).unwrap())
        }
        Commands::UpdateWorkshopItem { app_id, item_id } => {
            commands::update_workshop_item::update_workshop_item(app_id, item_id)
                .await
                .map(|_| "\"Workshop item update completed successfully\"".to_string())
        }
        Commands::SubscribedItems { app_id } => {
            commands::subscribed_items::subscribed_items(app_id)
                .await
                .map(|items| serde_json::to_string_pretty(&items).unwrap())
        }
        Commands::SearchWorkshop {
            app_id,
            query,
            max_results,
        } => commands::search_workshop::search_workshop(app_id, query, max_results)
            .await
            .map(|items| serde_json::to_string_pretty(&items).unwrap()),
        Commands::WorkshopPath { app_id } => match commands::workshop_path::workshop_path(app_id) {
            Some(path) => Ok(serde_json::to_string_pretty(&path).unwrap()),
            None => Err(format!("Workshop path not found for app ID {}", app_id)),
        },
        Commands::SteamLibraryPaths => commands::steam_library_paths::steam_library_paths()
            .map(|paths| serde_json::to_string_pretty(&paths).unwrap()),
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
