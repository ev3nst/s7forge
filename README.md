# S7Forge

A simple CLI utility for interacting with the Steam Workshop API. S7Forge provides basic commands for managing Steam Workshop content with local caching for improved performance.

## Installation

### Prerequisites

- Windows OS
- Rust 1.70+ 
- Steam SDK in the same directory as the binary (steam_api64.dll, steam_api64.lib)
- Steam account

### Build from Source

```bash
git clone <repository-url>
cd s7forge
cargo build --release
```

The compiled binary will be available at `target/release/s7forge.exe` (Windows) or `target/release/s7forge` (Linux/macOS).

## Usage

All commands require an `--app-id` parameter specifying the Steam application ID.

### Check Workshop Item Download Status

```bash
s7forge check-item-download --app-id 107410 --item-id 123456789
```

### Fetch Creator Names

```bash
# Single creator
s7forge fetch-creator-names --app-id 107410 --creator-ids 76561198000000000

# Multiple creators
s7forge fetch-creator-names --app-id 107410 --creator-ids 76561198000000000,76561198000000001
```

### Get Workshop Items

```bash
# Single item
s7forge get-workshop-items --app-id 107410 --item-ids 123456789

# Multiple items
s7forge get-workshop-items --app-id 107410 --item-ids 123456789,987654321,555666777
```

### Get Collection Items

```bash
s7forge get-collection-items --app-id 107410 --item-id 987654321
```

### Get Subscribed Items

```bash
s7forge get-subscribed-items --app-id 107410
```

### Subscribe to Workshop Item

```bash
s7forge subscribe --app-id 107410 --item-id 123456789
```

### Unsubscribe from Workshop Item

```bash
s7forge unsubscribe --app-id 107410 --item-id 123456789
```

### Update Workshop Item

```bash
s7forge update-workshop-item --app-id 107410 --item-id 123456789
```

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

## Caching

S7Forge uses simple local caching to improve performance:

- **Creator Names**: Single global cache file (`creator_names_cache.bin`)
- **Workshop Items**: Single global cache file (`workshop_items_cache.bin`)
- **Cache Location**: `{executable_directory}/cache/`

The cache grows over time, storing previously queried data for faster retrieval.

## License

This project is licensed under the MIT License.

## Credits

- [steamworks.js](https://github.com/ceifa/steamworks.js) - Some code portions adapted from this project (MIT License)

## Contributing

Contributions are welcome! Please ensure:

1. Code follows Rust conventions
2. Formatting is applied: `cargo fmt`

## Support

For issues, feature requests, or questions, please open an issue on the project repository.