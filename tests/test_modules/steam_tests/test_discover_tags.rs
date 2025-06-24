use crate::test_modules::utils::{assert_valid_json, run_command, steam_test_or_skip, TestConfig};
use std::collections::HashSet;

#[test]
fn test_discover_tags() {
    steam_test_or_skip(|| {
        let config = TestConfig::load();
        let output = run_command(&["discover-tags", "--app-id", &config.app_id.to_string()]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let value = assert_valid_json(&stdout);
            assert!(value.is_array(), "Expected JSON array, got: {}", stdout);

            let tags_array = value.as_array().unwrap();
            assert!(
                !tags_array.is_empty(),
                "Expected at least one tag to be discovered"
            );

            for (index, tag) in tags_array.iter().enumerate() {
                assert!(
                    tag.is_string(),
                    "Expected tag at index {} to be a string",
                    index
                );
                let tag_str = tag.as_str().unwrap();
                assert!(
                    !tag_str.is_empty(),
                    "Expected non-empty tag at index {}",
                    index
                );
                assert!(
                    !tag_str.trim().is_empty(),
                    "Expected non-whitespace tag at index {}",
                    index
                );
            }

            let mut unique_tags = HashSet::new();
            for tag in tags_array {
                let tag_str = tag.as_str().unwrap();
                assert!(
                    unique_tags.insert(tag_str),
                    "Found duplicate tag: {}",
                    tag_str
                );
            }

            println!(
                "✓ Discovered {} unique tags: {}",
                tags_array.len(),
                tags_array
                    .iter()
                    .take(5)
                    .map(|t| t.as_str().unwrap())
                    .collect::<Vec<_>>()
                    .join(", ")
            );

            if tags_array.len() > 5 {
                println!("  ... and {} more", tags_array.len() - 5);
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Discover tags test failed: {}", stderr);
        }
    });
}
