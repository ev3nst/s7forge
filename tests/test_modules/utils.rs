use s7forge::test_config::TestConfig;
use std::process::Command;

pub fn run_command(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute command")
}

pub fn is_steam_available() -> bool {
    let config = TestConfig::load();
    let output = run_command(&["workshop-path", "--app-id", &config.app_id.to_string()]);
    output.status.success()
}

pub fn assert_valid_json(json_str: &str) -> serde_json::Value {
    serde_json::from_str(json_str).expect(&format!("Expected valid JSON, got: {}", json_str))
}

pub fn assert_json_array(json_str: &str) {
    let value = assert_valid_json(json_str);
    assert!(value.is_array(), "Expected JSON array, got: {}", json_str);
}

pub fn steam_test_or_skip<F>(test_fn: F)
where
    F: FnOnce(),
{
    if is_steam_available() {
        test_fn();
    } else {
        println!("Skipping test: Steam not available");
    }
}

pub fn is_item_subscribed(app_id: u32, item_id: u64) -> Result<bool, String> {
    let output = run_command(&["subscribed-items", "--app-id", &app_id.to_string()]);

    if !output.status.success() {
        return Err("Failed to get subscribed items".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let items: Result<Vec<serde_json::Value>, _> = serde_json::from_str(&stdout);

    match items {
        Ok(items_array) => {
            for item in items_array {
                if let Some(id) = item.get("published_file_id") {
                    if let Some(id_num) = id.as_u64() {
                        if id_num == item_id {
                            return Ok(true);
                        }
                    }
                }
            }
            Ok(false)
        }
        Err(_) => Err("Failed to parse subscribed items JSON".to_string()),
    }
}
