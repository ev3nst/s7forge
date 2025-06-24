mod commands;
mod core;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "s7forge")]
#[command(about = "Steam utility")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check download status of a workshop item
    ///
    /// Example: s7forge check-item-download --app-id 548430 --item-id 123456789
    #[command(name = "check-item-download")]
    CheckItemDownload {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
        /// Workshop item ID to check
        #[arg(long, help = "Workshop item ID to check download status for")]
        item_id: u64,
    },
    /// Get items from a workshop collection
    ///
    /// Example: s7forge collection-items --app-id 548430 --item-id 987654321
    #[command(name = "collection-items")]
    CollectionItems {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
        /// Collection ID to fetch items from
        #[arg(long, help = "Collection ID to get items from")]
        item_id: u64,
    },
    /// Get detailed information about workshop items
    ///
    /// Example: s7forge workshop-items --app-id 548430 --item-ids 123,456,789
    #[command(name = "workshop-items")]
    WorkshopItems {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
        /// Comma-separated list of workshop item IDs
        #[arg(
            long,
            value_delimiter = ',',
            help = "Workshop item IDs (comma-separated)"
        )]
        item_ids: Vec<u64>,
    },
    /// Subscribe to workshop items
    ///
    /// Example: s7forge subscribe --app-id 548430 --item-ids 123,456,789
    Subscribe {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
        /// Comma-separated list of workshop item IDs to subscribe to
        #[arg(
            long,
            value_delimiter = ',',
            help = "Workshop item IDs to subscribe to (comma-separated)"
        )]
        item_ids: Vec<u64>,
    },
    /// Unsubscribe from workshop items
    ///
    /// Example: s7forge unsubscribe --app-id 548430 --item-ids 123,456,789
    Unsubscribe {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
        /// Comma-separated list of workshop item IDs to unsubscribe from
        #[arg(
            long,
            value_delimiter = ',',
            help = "Workshop item IDs to unsubscribe from (comma-separated)"
        )]
        item_ids: Vec<u64>,
    },
    /// Download a workshop item you own
    ///
    /// Example: s7forge download-workshop-item --app-id 548430 --item-id 123456789
    #[command(name = "download-workshop-item")]
    DownloadWorkshopItem {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
        /// Workshop item ID to download
        #[arg(long, help = "Workshop item ID to download")]
        item_id: u64,
    },
    /// List all items you're subscribed to for a game
    ///
    /// Example: s7forge subscribed-items --app-id 548430
    #[command(name = "subscribed-items")]
    SubscribedItems {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
    },
    /// Search workshop content by text query with flexible sorting options
    ///
    /// Example: s7forge search-workshop --app-id 548430 --query "tank" --sort-by relevance
    /// Example: s7forge search-workshop --app-id 548430 --sort-by recent --tags "mod,weapon"
    /// Example: s7forge search-workshop --app-id 548430 --sort-by popular --period one-week
    #[command(name = "search-workshop")]
    SearchWorkshop {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
        /// Search query text (optional for non-relevance sorting)
        #[arg(
            long,
            default_value = "",
            help = "Text to search for in workshop items (optional for most sort methods)"
        )]
        query: String,
        /// How to sort/rank the results
        #[arg(
            long,
            default_value = "relevance",
            value_parser = ["relevance", "recent", "popular", "most-subscribed", "recently-updated"],
            help = "Sort results by: relevance, recent, popular, most-subscribed, recently-updated"
        )]
        sort_by: String,
        /// Time period filter for supported sort types (popular only)
        #[arg(
            long,
            value_parser = ["today", "one-week", "three-months", "six-months", "one-year"],
            help = "Time period filter: today, one-week, three-months, six-months, one-year (only applies to 'popular' sort)"
        )]
        period: Option<String>,
        /// Page number for pagination (1-based)
        #[arg(
            long,
            default_value = "1",
            help = "Page number for pagination, starting from 1 (default: 1)"
        )]
        page: u32,
        /// Filter by tags (comma-separated)
        #[arg(
            long,
            help = "Filter results by tags, comma-separated (e.g., 'mod,weapon')"
        )]
        tags: Option<String>,
    },
    /// Get the local workshop path for a game
    ///
    /// Example: s7forge workshop-path --app-id 548430
    #[command(name = "workshop-path")]
    WorkshopPath {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
        app_id: u32,
    },
    /// List all Steam library folder paths
    ///
    /// Example: s7forge steam-library-paths
    #[command(name = "steam-library-paths")]
    SteamLibraryPaths,
    /// Clear all cached data (creator names, workshop items)
    ///
    /// Example: s7forge clear-cache
    #[command(name = "clear-cache")]
    ClearCache,
    /// Discover all available workshop tags for a game
    ///
    /// Example: s7forge discover-tags --app-id 548430
    #[command(name = "discover-tags")]
    DiscoverTags {
        /// Steam App ID (e.g., 548430 for Deep Rock Galactic)
        #[arg(long, help = "Steam App ID of the game")]
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
        Commands::DownloadWorkshopItem { app_id, item_id } => {
            commands::download_workshop_item::download_workshop_item(app_id, item_id)
                .await
                .map(|_| "\"Workshop item download completed successfully\"".to_string())
        }
        Commands::SubscribedItems { app_id } => {
            commands::subscribed_items::subscribed_items(app_id)
                .await
                .map(|items| serde_json::to_string_pretty(&items).unwrap())
        }
        Commands::SearchWorkshop {
            app_id,
            query,
            sort_by,
            period,
            page,
            tags,
        } => commands::search_workshop::search_workshop(app_id, query, sort_by, period, page, tags)
            .await
            .map(|items| serde_json::to_string_pretty(&items).unwrap()),
        Commands::WorkshopPath { app_id } => match commands::workshop_path::workshop_path(app_id) {
            Some(path) => Ok(serde_json::to_string_pretty(&path).unwrap()),
            None => Err(format!("Workshop path not found for app ID {}", app_id)),
        },
        Commands::SteamLibraryPaths => commands::steam_library_paths::steam_library_paths()
            .map(|paths| serde_json::to_string_pretty(&paths).unwrap()),
        Commands::ClearCache => commands::clear_cache::clear_cache()
            .map(|message| serde_json::to_string_pretty(&message).unwrap()),
        Commands::DiscoverTags { app_id } => commands::discover_tags::discover_tags(app_id)
            .await
            .map(|tags| serde_json::to_string_pretty(&tags).unwrap()),
    };

    match result {
        Ok(output) => {
            println!("{}", output);
            std::process::exit(0);
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1);
        }
    }
}
