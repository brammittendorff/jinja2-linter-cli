// Standard library imports are grouped together
use std::collections::HashSet;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

// External crate imports are done separately
use walkdir::{DirEntry, WalkDir};

// Since we're operating in an async context, we need this function to be async as well.
pub async fn scan_for_files(dir: &Path, allowed_extensions: HashSet<OsString>) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        // Now we check if the file has an allowed extension
        if is_file_matching(&entry, &allowed_extensions) {
            files.push(entry.into_path());
        }
    }

    Ok(files)
}

fn is_file_matching(
    entry: &DirEntry, 
    allowed_extensions: &HashSet<OsString>,
) -> bool {
    // Check if the entry is a file
    if entry.file_type().is_file() {
        // Check if the file has an allowed extension
        if let Some(ext) = entry.path().extension() {
            return allowed_extensions.contains(ext);
        }
    }
    false
}
