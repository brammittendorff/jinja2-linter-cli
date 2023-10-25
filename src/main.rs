use std::collections::{HashMap, HashSet}; // Used for 'HashMap' and 'HashSet'
use std::ffi::{OsStr, OsString}; // Used for 'OsStr' and 'OsString'
use std::io; // Used for 'io::Error'
use std::path::Path; // Used for '&Path'

use serde::Deserialize; // Used for the 'Deserialize' trait
use tokio; // Used for asynchronous operations
use toml; // Used for parsing TOML files
use clap::{App, Arg};

mod file_scanner; // Importing the file_scanner module

// Use a HashMap to represent the tool configurations dynamically.
#[derive(Debug, Deserialize)]
struct PyProject {
    #[allow(dead_code)] // This attribute suppresses the warning specifically for this field
    tool: HashMap<String, toml::Value>,
}

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

    // Existing logic for reading the configuration file...
    let _pyproject_config = match read_pyproject(config_path).await {
        Ok(config) => {
            println!("Configurations: {:?}", config);
            config
        },
        Err(e) => {
            eprintln!("Error reading config: {}", e);
            return;
        }
    };

    // Define your allowed extensions.
    let allowed_extensions: HashSet<OsString> = [
        OsStr::new("html"),
        OsStr::new("j2"),
        OsStr::new("jinja2"),
        OsStr::new("tmpl"),
        OsStr::new("jinja"),
        OsStr::new("j2t"),
    ]
    .iter()
    .cloned()
    .map(OsString::from)
    .collect();

    // Scan all files, considering the allowed extensions.
    let all_files = match file_scanner::scan_for_files(current_directory, allowed_extensions).await {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error scanning files: {}", e);
            return;
        }
    };

    // Process the files obtained from the scan. This is where you would add your linting or processing logic.
    for file in all_files {
        println!("Processing file: {:?}", file);

        // Insert your logic for each file here. 
        // For example, you could pass each file to an asynchronous linting function.
        //
        // Example:
        // match your_async_linting_function(file).await {
        //     Ok(_) => println!("Successfully linted {:?}", file),
        //     Err(err) => eprintln!("Error linting file {:?}: {}", file, err),
        // }
    }

    // Any further logic for after processing the files can go here.
}
