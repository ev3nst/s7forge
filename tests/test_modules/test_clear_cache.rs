use crate::test_modules::utils::{assert_valid_json, run_command};

#[test]
fn test_clear_cache() {
    let output = run_command(&["clear-cache"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let value = assert_valid_json(&stdout);

    assert!(
        value.is_object(),
        "Expected JSON object for clear-cache result"
    );

    let success = value.get("success").expect("Missing 'success' field");
    assert!(success.is_boolean(), "Expected 'success' to be a boolean");
    assert_eq!(
        success.as_bool().unwrap(),
        true,
        "Expected cache clearing to succeed"
    );

    let message = value.get("message").expect("Missing 'message' field");
    assert!(message.is_string(), "Expected 'message' to be a string");
    let message_str = message.as_str().unwrap();
    assert!(!message_str.is_empty(), "Expected non-empty message");

    let files_cleared = value
        .get("files_cleared")
        .expect("Missing 'files_cleared' field");
    assert!(
        files_cleared.is_number(),
        "Expected 'files_cleared' to be a number"
    );
    let cleared_count = files_cleared.as_u64().unwrap();

    let files = value.get("files").expect("Missing 'files' field");
    assert!(files.is_array(), "Expected 'files' to be an array");
    let files_array = files.as_array().unwrap();
    assert_eq!(
        files_array.len() as u64,
        cleared_count,
        "files array length should match files_cleared count"
    );

    for (index, file) in files_array.iter().enumerate() {
        assert!(
            file.is_string(),
            "Expected file at index {} to be a string",
            index
        );
        let file_str = file.as_str().unwrap();
        assert!(
            !file_str.is_empty(),
            "Expected non-empty filename at index {}",
            index
        );
    }

    if let Some(errors) = value.get("errors") {
        assert!(
            errors.is_array(),
            "Expected 'errors' to be an array if present"
        );
    }
}
