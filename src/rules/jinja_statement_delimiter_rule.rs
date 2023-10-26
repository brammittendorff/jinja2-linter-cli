use regex::Regex;
use std::path::PathBuf;
use tokio::fs;  // for async file reading

// You might have a custom error type for your application. For simplicity, we're using `Box<dyn std::error::Error>` here.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)] // This line is added to derive the Clone trait for the struct
pub struct JinjaStatementDelimiterRule {
    // Add fields if necessary, e.g., for configuration
}

impl JinjaStatementDelimiterRule {
    pub fn new() -> Self {
        // Initialize and return an instance of the rule.
        JinjaStatementDelimiterRule {
            // Initialization fields if you have any
        }
    }

    pub async fn check_file(&self, file_path: PathBuf) -> Result<()> {
        // Asynchronously read the file content
        let content = fs::read_to_string(&file_path).await?;

        // We'll define the regex pattern for wrong delimiters outside of the loop,
        // so it's only compiled once.
        let re_wrong_delimiters = Regex::new(r"\{%[\-\+]|[\-\+]%\}").unwrap();

        // Process the content (for example, by checking each line)
        for (line_no, line) in content.lines().enumerate() {
            // Check if any line matches the wrong delimiters pattern
            if re_wrong_delimiters.is_match(&line) {
                // Here, we're just printing the issues. 
                // You might want to collect these and return them for a summary report.
                println!(
                    "File {:?}, line {}: Jinja statements should not have wrong delimiters.",
                    file_path, line_no + 1
                );
            }
        }
        Ok(())
    }
}
