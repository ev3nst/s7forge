use crate::test_modules::utils::{assert_valid_json, run_command};
use s7forge::test_config::TestConfig;

#[test]
fn test_workshop_path_non_steam() {
    let config = TestConfig::load();
    let output = run_command(&["workshop-path", "--app-id", &config.app_id.to_string()]);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_valid_json(&stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Error:"));
    }
}
