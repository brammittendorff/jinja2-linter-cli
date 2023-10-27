use tokio::fs;
use std::io;
use std::path::PathBuf;
use std::collections::HashSet;
use std::ffi::OsString;
use std::sync::Arc;
use tokio::sync::Semaphore;
use std::pin::Pin;
use std::future::Future;

pub fn scan_for_files(
    dir: PathBuf, // Owned PathBuf because we're in async context
    allowed_extensions: Arc<HashSet<OsString>>, // Arc used because we can't have references in async blocks
    semaphore: Arc<Semaphore>,
) -> Pin<Box<dyn Future<Output = io::Result<Vec<PathBuf>>> + Send>> {
    Box::pin(async move {
        let mut files_to_process = Vec::new();

        // Since this is an async function, we should handle the potential error instead of unwrapping.
        let mut entries = match fs::read_dir(dir).await {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };

        // We will store our directory scanning tasks here.
        let mut tasks = Vec::new();

        // Iterate over the directory entries.
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();

            // If this is a directory, we will scan it by spawning a new task.
            if path.is_dir() {
                let semaphore_clone = Arc::clone(&semaphore);
                let allowed_extensions_clone = Arc::clone(&allowed_extensions);

                // Recursive call is now through the boxed future.
                let task = tokio::spawn(scan_for_files(
                    path,
                    allowed_extensions_clone,
                    semaphore_clone,
                ));

                tasks.push(task);
            } else {
                // If the path has a valid extension, which is allowed, add it to the processing list.
                if let Some(ext) = path.extension() {
                    if allowed_extensions.contains(ext) {
                        files_to_process.push(path);
                    }
                }
            }
        }

        // Now we await the tasks and collect all valid files from the subdirectories.
        for task in tasks {
            if let Ok(Ok(mut subdir_files)) = task.await {
                files_to_process.append(&mut subdir_files);
            }
            // You might want to add error handling here, depending on your requirements.
        }

        Ok(files_to_process)
    })
}
