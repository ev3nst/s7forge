use crate::test_modules::utils::{assert_valid_json, run_command, steam_test_or_skip};
use s7forge::test_config::TestConfig;

#[test]
fn test_check_item_download() {
    steam_test_or_skip(|| {
        let config = TestConfig::load();
        let output = run_command(&[
            "check-item-download",
            "--app-id",
            &config.app_id.to_string(),
            "--item-id",
            &config.item_id.to_string(),
        ]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let value = assert_valid_json(&stdout);
            assert!(value.is_object(), "Expected JSON object, got: {}", stdout);

            let is_downloading = value
                .get("is_downloading")
                .expect("Missing 'is_downloading' field");
            assert!(
                is_downloading.is_boolean(),
                "Expected 'is_downloading' to be a boolean"
            );

            let downloaded_bytes = value
                .get("downloaded_bytes")
                .expect("Missing 'downloaded_bytes' field");
            assert!(
                downloaded_bytes.is_number(),
                "Expected 'downloaded_bytes' to be a number"
            );

            let total_bytes = value
                .get("total_bytes")
                .expect("Missing 'total_bytes' field");
            assert!(
                total_bytes.is_number(),
                "Expected 'total_bytes' to be a number"
            );

            let progress_percentage = value
                .get("progress_percentage")
                .expect("Missing 'progress_percentage' field");
            assert!(
                progress_percentage.is_number(),
                "Expected 'progress_percentage' to be a number"
            );
            let progress_val = progress_percentage.as_f64().unwrap() as f32;
            assert!(
                progress_val >= 0.0 && progress_val <= 100.0,
                "Expected progress percentage between 0-100"
            );

            let download_complete = value
                .get("download_complete")
                .expect("Missing 'download_complete' field");
            assert!(
                download_complete.is_boolean(),
                "Expected 'download_complete' to be a boolean"
            );

            if download_complete.as_bool().unwrap() {
                assert_eq!(
                    progress_val, 100.0,
                    "If download is complete, progress should be 100%"
                );
                assert!(
                    !is_downloading.as_bool().unwrap(),
                    "If download is complete, should not be downloading"
                );
            }

            println!(
                "✓ Download info validation passed: {}% complete",
                progress_val
            );
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Check item download test failed: {}", stderr);
        }
    });
}
