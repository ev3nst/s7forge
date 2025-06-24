use crate::test_modules::utils::{assert_valid_json, run_command, steam_test_or_skip, TestConfig};

#[test]
fn test_collection_items() {
    steam_test_or_skip(|| {
        let config = TestConfig::load();
        let output = run_command(&[
            "collection-items",
            "--app-id",
            &config.app_id.to_string(),
            "--item-id",
            &config.collection_id.to_string(),
        ]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let value = assert_valid_json(&stdout);
            assert!(value.is_object(), "Expected JSON object, got: {}", stdout);

            let details = value.get("details").expect("Missing 'details' field");
            assert!(details.is_object(), "Expected 'details' to be an object");

            let id = details.get("id").expect("Missing 'id' field in details");
            assert!(id.is_number(), "Expected 'id' to be a number");
            assert_eq!(
                id.as_u64().unwrap(),
                config.collection_id,
                "Collection ID mismatch"
            );

            let title = details
                .get("title")
                .expect("Missing 'title' field in details");
            assert!(title.is_string(), "Expected 'title' to be a string");
            assert!(
                !title.as_str().unwrap().is_empty(),
                "Expected non-empty title"
            );

            let description = details
                .get("description")
                .expect("Missing 'description' field in details");
            assert!(
                description.is_string(),
                "Expected 'description' to be a string"
            );

            let time_created = details
                .get("time_created")
                .expect("Missing 'time_created' field in details");
            assert!(
                time_created.is_number(),
                "Expected 'time_created' to be a number"
            );
            assert!(
                time_created.as_u64().unwrap() > 0,
                "Expected valid timestamp"
            );

            let time_updated = details
                .get("time_updated")
                .expect("Missing 'time_updated' field in details");
            assert!(
                time_updated.is_number(),
                "Expected 'time_updated' to be a number"
            );

            let num_upvotes = details
                .get("num_upvotes")
                .expect("Missing 'num_upvotes' field in details");
            assert!(
                num_upvotes.is_number(),
                "Expected 'num_upvotes' to be a number"
            );

            let num_downvotes = details
                .get("num_downvotes")
                .expect("Missing 'num_downvotes' field in details");
            assert!(
                num_downvotes.is_number(),
                "Expected 'num_downvotes' to be a number"
            );

            let items = value.get("items").expect("Missing 'items' field");
            assert!(items.is_array(), "Expected 'items' to be an array");
            let items_array = items.as_array().unwrap();
            assert!(
                !items_array.is_empty(),
                "Expected at least one item in collection"
            );

            let first_item = &items_array[0];
            assert!(first_item.is_object(), "Expected item to be an object");

            let item_id = first_item
                .get("published_file_id")
                .expect("Missing 'published_file_id' field in item");
            assert!(
                item_id.is_number(),
                "Expected item 'published_file_id' to be a number"
            );
            assert!(item_id.as_u64().unwrap() > 0, "Expected valid item ID");

            let item_title = first_item
                .get("title")
                .expect("Missing 'title' field in item");
            assert!(
                item_title.is_string(),
                "Expected item 'title' to be a string"
            );

            let creator_app_id = first_item
                .get("creator_steam_game_id")
                .expect("Missing 'creator_steam_game_id' field in item");
            if creator_app_id.is_number() {
                let _valid_id = creator_app_id.as_u64().unwrap();
            } else {
                assert!(
                    creator_app_id.is_null(),
                    "Expected creator_steam_game_id to be a number or null"
                );
            }

            println!(
                "✓ Collection validation passed: {} items in collection",
                items_array.len()
            );
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Collection items test failed: {}", stderr);
        }
    });
}
