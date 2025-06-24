use crate::test_modules::utils::{assert_valid_json, run_command};

#[test]
fn test_steam_library_paths() {
    let output = run_command(&["steam-library-paths"]);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert_valid_json(&stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Error:"));
    }
}
