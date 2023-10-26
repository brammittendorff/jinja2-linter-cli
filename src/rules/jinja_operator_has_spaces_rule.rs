use regex::Regex;
use std::path::PathBuf;
use tokio::fs;  // for async file reading

// You might have a custom error type for your application.
// For simplicity, we're going to use `Box<dyn std::error::Error>` here.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)] // This line is added to derive the Clone trait
pub struct JinjaOperatorHasSpacesRule {
    // Add fields if necessary, e.g., for configuration
}

impl JinjaOperatorHasSpacesRule {
    pub fn new() -> Self {
        // Initialize and return an instance of the rule.
        JinjaOperatorHasSpacesRule {
            // Initialization fields if you have any
        }
    }

    pub async fn check_file(&self, file_path: PathBuf) -> Result<()> {
        // Asynchronously read the file content
        let content = fs::read_to_string(&file_path).await?;

        // Process the content (for example, by checking each line)
        for (line_no, line) in content.lines().enumerate() {
            // Simple example of checking for a missing space around an operator "|"
            // Note: This is a simplified regex and might not cover complex Jinja syntax cases
            let re = Regex::new(r"\{\{[^{}]*\|[^{}]*\}\}").unwrap(); // Adjust based on actual needs
            if re.is_match(&line) && !Regex::new(r"\{\{[^{}]*\s\|\s[^{}]*\}\}").unwrap().is_match(&line) {
                // For the sake of this example, we're just printing the issues. 
                // You might want to collect these and return them for a summary report.
                println!(
                    "File {:?}, line {}: Operator '|' should be enclosed by spaces.",
                    file_path, line_no + 1
                );
            }
        }
        Ok(())
    }
}
