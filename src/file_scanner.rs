use std::collections::HashSet;
use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs; // Make sure to use async fs from tokio
use std::future::Future;
use std::pin::Pin;

pub fn scan_for_files(
    dir: &Path, 
    allowed_extensions: HashSet<OsString>
) -> Pin<Box<dyn Future<Output = io::Result<Vec<PathBuf>>> + '_>> {
    Box::pin(async move {
        let mut files_to_process = Vec::new();

        // Recursive directory traversal with async reading
        let mut read_dir = fs::read_dir(dir).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                // Recursive call to handle subdirectories
                let mut subdir_files = scan_for_files(&path, allowed_extensions.clone()).await?;
                files_to_process.append(&mut subdir_files);
            } else {
                if let Some(ext) = path.extension() {
                    if allowed_extensions.contains(ext) {
                        files_to_process.push(path);
                    }
                }
            }
        }

        Ok(files_to_process)
    })
}
