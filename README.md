# s7forge

A simple CLI utility for interacting with the Steam Workshop API. S7Forge provides commands for managing Steam Workshop content with local caching for improved performance.

## Installation

### Prerequisites

- Windows OS (current implementation)
- Rust 1.70+ 
- Steam SDK in the same directory as the binary (steam_api64.dll, steam_api64.lib)
- Steam account and Steam client running

### Build from Source

```bash
git clone <repository-url>
cd s7forge
cargo build --release
```

The compiled binary will be available at `target/release/s7forge.exe`.

## Usage

All commands that interact with the Steam Workshop require an `--app-id` parameter specifying the Steam application ID (e.g., 548430 for Deep Rock Galactic). Use `s7forge <command> --help` to see detailed help for any command.

### Help and Command Discovery

```bash
# Show all available commands
s7forge --help

# Show help for a specific command
s7forge check-item-download --help
```

### Check Workshop Item Download Status

```bash
s7forge check-item-download --app-id 548430 --item-id 123456789
```

### Get Items from Workshop Collection

```bash
s7forge collection-items --app-id 548430 --item-id 987654321
```

### Get Workshop Items Information

```bash
# Single item
s7forge workshop-items --app-id 548430 --item-ids 123456789

# Multiple items
s7forge workshop-items --app-id 548430 --item-ids 123456789,987654321,555666777
```

### Subscribe to Workshop Items

```bash
# Single item
s7forge subscribe --app-id 548430 --item-ids 123456789

# Multiple items (batch subscription)
s7forge subscribe --app-id 548430 --item-ids 123456789,987654321,555666777
```

### Unsubscribe from Workshop Items

```bash
# Single item
s7forge unsubscribe --app-id 548430 --item-ids 123456789

# Multiple items (batch unsubscription)
s7forge unsubscribe --app-id 548430 --item-ids 123456789,987654321,555666777
```

### Update Workshop Item

```bash
s7forge update-workshop-item --app-id 548430 --item-id 123456789
```

### Get Subscribed Items

```bash
s7forge subscribed-items --app-id 548430
```

### Search Workshop Content

```bash
# Basic search
s7forge search-workshop --app-id 548430 --query "tank"

# Search with custom result limit
s7forge search-workshop --app-id 548430 --query "mining equipment" --max-results 20
```

### Get Workshop Path

```bash
s7forge workshop-path --app-id 548430
```

### Get Steam Library Paths

```bash
s7forge steam-library-paths
```

### Clear Cache

```bash
s7forge clear-cache
```

### Batch Operations

S7Forge supports efficient batch operations for multiple workshop items:

```bash
# Subscribe to multiple items at once
s7forge subscribe --app-id 548430 --item-ids 123,456,789,101112

# Unsubscribe from multiple items at once  
s7forge unsubscribe --app-id 548430 --item-ids 123,456,789

# Get information for multiple items
s7forge workshop-items --app-id 548430 --item-ids 123,456,789,101112
```

Batch operations are more efficient than individual calls and provide consolidated status reporting.

### Example Workshop Item Output

```json
[
  {
    "published_file_id": "number",
    "creator_steam_game_id": "number | null",
    "creator_name": "string",
    "consumer_steam_game_id": "number | null",
    "title": "string",
    "description": "string",
    "owner": {
      "steam_id64": "number",
      "steam_id32": "string",
      "account_id": "number"
    },
    "time_created": "number",
    "time_updated": "number",
    "time_added_to_user_list": "number",
    "visibility": "string",
    "banned": "boolean",
    "accepted_for_use": "boolean",
    "tags": "string",
    "tags_truncated": "boolean",
    "url": "string",
    "num_upvotes": "number",
    "num_downvotes": "number",
    "num_children": "number",
    "preview_url": "string | null",
    "statistics": {
      "num_subscriptions": "number | null",
      "num_favorites": "number | null",
      "num_followers": "number | null",
      "num_unique_subscriptions": "number | null",
      "num_unique_favorites": "number | null",
      "num_unique_followers": "number | null",
      "num_unique_website_views": "number | null",
      "report_score": "number | null",
      "num_seconds_played": "number | null",
      "num_playtime_sessions": "number | null",
      "num_comments": "number | null",
      "num_seconds_played_during_time_period": "number | null",
      "num_playtime_sessions_during_time_period": "number | null"
    },
    "required_items": "array of numbers",
    "file_type": "string",
    "file_size": "number"
  }
]
```

### Caching

S7Forge uses local caching to improve performance and reduce API calls:

- **Creator Names**: Cached globally (`creator_names_cache.bin`)
- **Workshop Items**: Cached globally (`workshop_items_cache.bin`)
- **Cache Location**: `{executable_directory}/cache/`
- **Cache Management**: Use `s7forge clear-cache` to clear all cached data

The cache grows incrementally, storing previously queried data for faster subsequent retrieval.

## Project Status

S7Forge is in active development. Current version focuses on Windows support with the Steam SDK.

## License

This project is licensed under the MIT License.

## Credits

- [steamworks.js](https://github.com/ceifa/steamworks.js) - Reference implementation (MIT License)
- [steamworks-rs](https://github.com/Noxime/steamworks-rs) - Rust Steam API bindings

## Contributing

Contributions are welcome! Please ensure:

1. Code follows Rust conventions and passes `cargo clippy`
2. Code is formatted with `cargo fmt`
3. New features include appropriate error handling
4. Changes maintain backward compatibility where possible

## Support

For issues, feature requests, or questions, please open an issue on the project repository.