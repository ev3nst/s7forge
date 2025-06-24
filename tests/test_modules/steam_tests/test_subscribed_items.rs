use crate::test_modules::utils::{assert_valid_json, run_command, steam_test_or_skip, TestConfig};

#[test]
fn test_subscribed_items() {
    steam_test_or_skip(|| {
        let config = TestConfig::load();
        let output = run_command(&["subscribed-items", "--app-id", &config.app_id.to_string()]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let value = assert_valid_json(&stdout);
            assert!(value.is_array(), "Expected JSON array, got: {}", stdout);

            let items_array = value.as_array().unwrap();
            println!("✓ Found {} subscribed items", items_array.len());

            if !items_array.is_empty() {
                let first_item = &items_array[0];
                assert!(
                    first_item.is_object(),
                    "Expected subscribed item to be an object"
                );

                let published_file_id = first_item
                    .get("published_file_id")
                    .expect("Missing 'published_file_id' field");
                assert!(
                    published_file_id.is_number(),
                    "Expected 'published_file_id' to be a number"
                );
                assert!(
                    published_file_id.as_u64().unwrap() > 0,
                    "Expected valid item ID"
                );

                let title = first_item.get("title").expect("Missing 'title' field");
                assert!(title.is_string(), "Expected 'title' to be a string");

                let creator_name = first_item
                    .get("creator_name")
                    .expect("Missing 'creator_name' field");
                assert!(
                    creator_name.is_string(),
                    "Expected 'creator_name' to be a string"
                );

                let url = first_item.get("url").expect("Missing 'url' field");
                assert!(url.is_string(), "Expected 'url' to be a string");

                println!(
                    "✓ Subscribed items validation passed: First item '{}' by {}",
                    title.as_str().unwrap(),
                    creator_name.as_str().unwrap()
                );
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Subscribed items test failed: {}", stderr);
        }
    });
}
