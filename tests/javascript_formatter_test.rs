use neatify::{format, format_dir};
use std::fs;

#[test]
fn test_javascript_formatting() {
    // Create a temporary directory for testing
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.js");

    // Create a test JavaScript file with unformatted code
    let unformatted_js = r#"function test(){const x=5;if(x>3){return true;}else{return false;}}"#;
    fs::write(&file_path, unformatted_js).unwrap();

    // Format the file
    let formatted = format(&file_path, true).unwrap();
    assert!(formatted, "File should need formatting");

    // Read the formatted content
    let formatted_content = fs::read_to_string(&file_path).unwrap();

    // Check that the formatting was applied correctly
    assert!(formatted_content.contains("function test()"));
    assert!(formatted_content.contains("const x = 5"));
    assert!(formatted_content.contains("if (x > 3)"));

    // Format the file again - should return false as it's already formatted
    let formatted_again = format(&file_path, true).unwrap();
    assert!(!formatted_again, "File should not need formatting again");

    // Test directory formatting
    let stats = format_dir(temp_dir.path(), true).unwrap();
    assert_eq!(stats.total_files, 1, "Should process 1 file");
    assert_eq!(
        stats.formatted_files, 0,
        "Should not format any files as they're already formatted"
    );
}
