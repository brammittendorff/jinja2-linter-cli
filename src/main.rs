use std::collections::HashSet;
use std::ffi::OsString;
use std::io; // Used for 'io::Error'
use std::path::Path; // Used for '&Path'

use tokio; // Used for asynchronous operations
use toml; // Used for parsing TOML files
use clap::{App, Arg};

mod file_scanner; // Importing the file_scanner module

mod config; // Include the new config module
use config::PyProject; // Use the PyProject struct from the config module

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

    // Now, you can pass allowed_extensions_set to the function as it expects a HashSet
    let all_files = match file_scanner::scan_for_files(current_directory, allowed_extensions_set).await {
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
