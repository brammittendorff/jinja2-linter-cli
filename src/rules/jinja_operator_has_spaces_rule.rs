use regex::Regex;
use std::path::PathBuf;
use tokio::fs;
use std::sync::Arc;

// Assume you might have a custom error type for your application.
// For simplicity, we're going to use `Box<dyn std::error::Error>` here.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)] // This ensures the struct implements the Clone trait
pub struct JinjaOperatorHasSpacesRule {
    // Pre-compiled regex to be used for checks.
    regex_no_space: Arc<Regex>,
    regex_with_space: Arc<Regex>,
    // Add other fields as necessary, e.g., for configuration.
}

impl JinjaOperatorHasSpacesRule {
    pub fn new() -> Self {
        // Initialize and return an instance of the rule.
        // Precompile the regexes here, before we start scanning files.
        let regex_no_space = Arc::new(Regex::new(r"\{\{[^{}]*\|[^{}]*\}\}").unwrap());
        let regex_with_space = Arc::new(Regex::new(r"\{\{[^{}]*\s\|\s[^{}]*\}\}").unwrap());

        JinjaOperatorHasSpacesRule {
            regex_no_space,
            regex_with_space,
            // Initialize other fields if you have any.
        }
    }

    pub async fn check_file(&self, file_path: PathBuf) -> Result<()> {
        // Asynchronously read the file content
        let content = fs::read_to_string(&file_path).await?;

        // Process the content (for example, by checking each line)
        for (line_no, line) in content.lines().enumerate() {
            // Use the pre-compiled regexes from the struct's fields.
            // This avoids re-compiling the regexes every time, which is expensive.
            if self.regex_no_space.is_match(&line) && !self.regex_with_space.is_match(&line) {
                // Here, we're just printing the issues, but you might want to collect these
                // and return them for a summary report, or handle them as appropriate for your application.
                println!(
                    "File {:?}, line {}: Operator '|' should be enclosed by spaces.",
                    file_path, line_no + 1
                );
            }
        }
        Ok(())
    }
}
