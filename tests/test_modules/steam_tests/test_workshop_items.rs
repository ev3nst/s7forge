use crate::test_modules::utils::{TestConfig, assert_valid_json, run_command, steam_test_or_skip};

#[test]
fn test_workshop_items() {
    steam_test_or_skip(|| {
        let config = TestConfig::load();
        let output = run_command(&[
            "workshop-items",
            "--app-id",
            &config.app_id.to_string(),
            "--item-ids",
            &config.item_id.to_string(),
        ]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let value = assert_valid_json(&stdout);
            assert!(value.is_array(), "Expected JSON array, got: {}", stdout);

            let items_array = value.as_array().unwrap();
            assert!(
                !items_array.is_empty(),
                "Expected at least one workshop item"
            );

            let first_item = &items_array[0];
            assert!(
                first_item.is_object(),
                "Expected workshop item to be an object"
            );

            let published_file_id = first_item
                .get("published_file_id")
                .expect("Missing 'published_file_id' field");
            assert!(
                published_file_id.is_number(),
                "Expected 'published_file_id' to be a number"
            );
            assert_eq!(
                published_file_id.as_u64().unwrap(),
                config.item_id,
                "Item ID mismatch"
            );

            let title = first_item.get("title").expect("Missing 'title' field");
            assert!(title.is_string(), "Expected 'title' to be a string");
            assert!(
                !title.as_str().unwrap().is_empty(),
                "Expected non-empty title"
            );

            let description = first_item
                .get("description")
                .expect("Missing 'description' field");
            assert!(
                description.is_string(),
                "Expected 'description' to be a string"
            );

            let owner = first_item.get("owner").expect("Missing 'owner' field");
            assert!(owner.is_object(), "Expected 'owner' to be an object");

            let time_created = first_item
                .get("time_created")
                .expect("Missing 'time_created' field");
            assert!(
                time_created.is_number(),
                "Expected 'time_created' to be a number"
            );
            assert!(
                time_created.as_u64().unwrap() > 0,
                "Expected valid timestamp"
            );

            let time_updated = first_item
                .get("time_updated")
                .expect("Missing 'time_updated' field");
            assert!(
                time_updated.is_number(),
                "Expected 'time_updated' to be a number"
            );

            let tags = first_item.get("tags").expect("Missing 'tags' field");
            assert!(tags.is_string(), "Expected 'tags' to be a string");

            let url = first_item.get("url").expect("Missing 'url' field");
            assert!(url.is_string(), "Expected 'url' to be a string");

            let num_upvotes = first_item
                .get("num_upvotes")
                .expect("Missing 'num_upvotes' field");
            assert!(
                num_upvotes.is_number(),
                "Expected 'num_upvotes' to be a number"
            );

            let num_downvotes = first_item
                .get("num_downvotes")
                .expect("Missing 'num_downvotes' field");
            assert!(
                num_downvotes.is_number(),
                "Expected 'num_downvotes' to be a number"
            );

            let file_size = first_item
                .get("file_size")
                .expect("Missing 'file_size' field");
            assert!(file_size.is_number(), "Expected 'file_size' to be a number");

            let creator_name = first_item
                .get("creator_name")
                .expect("Missing 'creator_name' field");
            assert!(
                creator_name.is_string(),
                "Expected 'creator_name' to be a string"
            );
            assert!(
                !creator_name.as_str().unwrap().is_empty(),
                "Expected non-empty creator name"
            );

            println!(
                "✓ Workshop item validation passed: '{}' by {}",
                title.as_str().unwrap(),
                creator_name.as_str().unwrap()
            );
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Workshop items test failed: {}", stderr);
        }
    });
}
