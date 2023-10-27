use std::collections::HashSet;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use futures::future;

use tokio::sync::Semaphore;
use std::sync::Arc;
use toml;
use clap::{App, Arg};

mod file_scanner;
use file_scanner::scan_for_files;

mod config;
use config::PyProject;

mod rules;
use rules::jinja_operator_has_spaces_rule::JinjaOperatorHasSpacesRule;

// This function attempts to read and parse a pyproject.toml file.
async fn read_pyproject(file_path: &Path) -> Result<PyProject, io::Error> {
    let content = tokio::fs::read_to_string(file_path).await?;
    let pyproject: PyProject = toml::from_str(&content)?;
    Ok(pyproject)
}

// Required for the async main function.
#[tokio::main]
async fn main() {

    // Set up the command-line arguments using the `clap` crate.
    let matches = App::new("Jinja2 Linter CLI")
        .version("0.1")
        .author("Bram Mittendorff <botw44@gmail.com>")
        .about("This is a CLI linter for Jinja2 templates.")
        .arg(
            Arg::with_name("config")
                .short('c') // Change here: use a char, not a string
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("directory")
                .short('d') // Change here: use a char, not a string
                .long("directory")
                .value_name("DIR")
                .help("Sets the directory to scan")
                .takes_value(true),
        )
        .get_matches();

    // Get the values of the input arguments. If not provided, defaults are used.
    let config_path_str = matches.value_of("config").unwrap_or("./pyproject.toml");
    let directory_path_str = matches.value_of("directory").unwrap_or(".");

    let config_path = Path::new(config_path_str);
    let current_directory = Path::new(directory_path_str);
    let current_directory_buf = current_directory.to_path_buf();

    // Existing logic for reading the configuration file...
    let pyproject_config = match read_pyproject(config_path).await {
        Ok(config) => {
            println!("Configurations: {:?}", config);
            config
        },
        Err(e) => {
            eprintln!("Error reading config: {}", e);
            return;
        }
    };

    // Make sure pyproject_config is of type PyProject from config.rs
    let allowed_extensions = config::get_os_string_allowed_extensions(
        &pyproject_config.tool.jinja2_linter_cli.allowed_extensions,
    );

    // Before calling scan_for_files, convert allowed_extensions from Vec to HashSet
    let allowed_extensions_set: HashSet<OsString> = allowed_extensions.into_iter().collect();
    let allowed_extensions_arc = Arc::new(allowed_extensions_set);

    // Control the level of concurrency with a semaphore.
    let max_concurrent_tasks = 50; // This value can be adjusted according to the system's capabilities.
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));

    // Here we use the updated scan_for_files function and pass the needed arguments. 
    // Notice that we're passing a reference to the allowed_extensions_set to avoid cloning.
    let all_files = match scan_for_files(current_directory_buf, allowed_extensions_arc, semaphore.clone()).await {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error scanning files: {}", e);
            return;
        }
    };

    let rule1 = JinjaOperatorHasSpacesRule::new();

    // Preparing for concurrent task execution with proper error handling.
    let mut tasks = Vec::new();
    for file in all_files {
        let rule = rule1.clone(); // Cloning the rule for use in multiple tasks.
        tasks.push(tokio::spawn(async move {
            if let Err(err) = rule.check_file(file).await { // Passing reference to file path.
                eprintln!("Error checking file: {}", err);
            }
        }));
    }

    // Wait for all tasks to complete.
    let _ = future::join_all(tasks).await;

    // Any further logic for after processing the files can go here.
}
