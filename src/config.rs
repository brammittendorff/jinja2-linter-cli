// config.rs
use serde::Deserialize;
use std::ffi::OsString;

// Structs to match the expected configuration structure.
#[derive(Debug, Deserialize)]
pub struct Jinja2LinterCli {
    #[serde(default = "default_allowed_extensions")]
    pub allowed_extensions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tool {
    #[serde(rename = "jinja2-linter-cli")]
    pub jinja2_linter_cli: Jinja2LinterCli,
}

#[derive(Debug, Deserialize)]
pub struct PyProject {
    pub tool: Tool,
}

// Function to provide default values for allowed extensions
fn default_allowed_extensions() -> Vec<String> {
    vec![
        "html".into(),
        "j2".into(),
        "jinja2".into(),
        "tmpl".into(),
        "jinja".into(),
        "j2t".into(),
    ]
}

// This function converts the allowed extensions from Strings to OsStrings, suitable for file operations.
pub fn get_os_string_allowed_extensions(extensions: &[String]) -> Vec<OsString> {
    extensions.iter().map(|ext| OsString::from(ext)).collect()
}
