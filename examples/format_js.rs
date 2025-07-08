use anyhow::Result;
use neatify::{format, format_dir};

fn main() -> Result<()> {
    // Example JavaScript code to format
    let js_code = r#"function example(){const x=5;if(x>3){return true;}else{return false;}}
    const obj = {a:1,b:2,c:3};
    "#;

    // Create a temporary file
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("example.js");
    std::fs::write(&file_path, js_code)?;

    println!("Original file created at: {}", file_path.display());

    // Format the file
    let formatted = format(&file_path, true)?;
    println!("File formatted: {}", formatted);

    // Read and display the formatted content
    let formatted_content = std::fs::read_to_string(&file_path)?;
    println!("\nFormatted content:\n{}", formatted_content);

    // Format a directory (if it contains JavaScript files)
    println!("\nFormatting directory: {}", temp_dir.display());
    let stats = format_dir(&temp_dir, true)?;

    // Display formatting statistics
    println!("\nFormatting statistics:");
    println!("  Total files processed: {}", stats.total_files);
    println!("  Files formatted: {}", stats.formatted_files);
    println!(
        "  Files needing formatting: {}",
        stats.files_needing_formatting
    );

    // Clean up
    std::fs::remove_file(file_path)?;

    Ok(())
}
