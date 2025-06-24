use crate::test_modules::utils::run_command;

#[test]
fn test_discover_tags_help() {
    let output = run_command(&["discover-tags", "--help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("Discover all available workshop tags"),
        "Help should describe what discover-tags does"
    );

    assert!(
        stdout.contains("--app-id"),
        "Help should mention '--app-id' parameter"
    );

    assert!(
        stdout.contains("--help"),
        "Help should mention '--help' option"
    );
}
