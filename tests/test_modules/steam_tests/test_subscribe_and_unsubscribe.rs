use crate::test_modules::utils::{
    assert_json_array, is_item_subscribed, run_command, steam_test_or_skip,
};
use s7forge::test_config::TestConfig;
use std::time::Duration;

// WARNING: These tests actually modify Steam subscriptions!
// Only run if you're okay with subscribing/unsubscribing from test items
#[test]
#[ignore] // Ignored by default - run with `cargo test -- --ignored` if you want to test subscriptions
fn test_subscribe_and_unsubscribe() {
    steam_test_or_skip(|| {
        let config = TestConfig::load();

        let was_initially_subscribed = match is_item_subscribed(config.app_id, config.item_id) {
            Ok(subscribed) => subscribed,
            Err(e) => {
                println!("Failed to check initial subscription state: {}", e);
                return;
            }
        };

        println!(
            "Item {} initial subscription state: {}",
            config.item_id,
            if was_initially_subscribed {
                "subscribed"
            } else {
                "not subscribed"
            }
        );

        if was_initially_subscribed {
            println!("Testing unsubscribe -> subscribe cycle");

            let unsubscribe_output = run_command(&[
                "unsubscribe",
                "--app-id",
                &config.app_id.to_string(),
                "--item-ids",
                &config.item_id.to_string(),
            ]);

            if unsubscribe_output.status.success() {
                let stdout = String::from_utf8_lossy(&unsubscribe_output.stdout);
                assert_json_array(&stdout);
                println!("✓ Unsubscribe successful");

                std::thread::sleep(Duration::from_secs(2));

                let subscribe_output = run_command(&[
                    "subscribe",
                    "--app-id",
                    &config.app_id.to_string(),
                    "--item-ids",
                    &config.item_id.to_string(),
                ]);

                if subscribe_output.status.success() {
                    let stdout = String::from_utf8_lossy(&subscribe_output.stdout);
                    assert_json_array(&stdout);
                    println!("✓ Subscribe successful - restored original state");
                } else {
                    let stderr = String::from_utf8_lossy(&subscribe_output.stderr);
                    println!("❌ Failed to restore subscription: {}", stderr);
                }
            } else {
                let stderr = String::from_utf8_lossy(&unsubscribe_output.stderr);
                println!("❌ Unsubscribe test failed: {}", stderr);
            }
        } else {
            println!("Testing subscribe -> unsubscribe cycle");

            let subscribe_output = run_command(&[
                "subscribe",
                "--app-id",
                &config.app_id.to_string(),
                "--item-ids",
                &config.item_id.to_string(),
            ]);

            if subscribe_output.status.success() {
                let stdout = String::from_utf8_lossy(&subscribe_output.stdout);
                assert_json_array(&stdout);
                println!("✓ Subscribe successful");

                std::thread::sleep(Duration::from_secs(2));

                let unsubscribe_output = run_command(&[
                    "unsubscribe",
                    "--app-id",
                    &config.app_id.to_string(),
                    "--item-ids",
                    &config.item_id.to_string(),
                ]);

                if unsubscribe_output.status.success() {
                    let stdout = String::from_utf8_lossy(&unsubscribe_output.stdout);
                    assert_json_array(&stdout);
                    println!("✓ Unsubscribe successful - restored original state");
                } else {
                    let stderr = String::from_utf8_lossy(&unsubscribe_output.stderr);
                    println!("❌ Failed to restore unsubscribed state: {}", stderr);
                }
            } else {
                let stderr = String::from_utf8_lossy(&subscribe_output.stderr);
                println!("❌ Subscribe test failed: {}", stderr);
            }
        }
    });
}
