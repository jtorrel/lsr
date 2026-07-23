use crate::entry::Entry;
use crate::entry::EntryKind;
use crate::errors::LsrError;
use crate::options::Options;

use chrono::{DateTime, Local};
use std::path::Path;

pub fn list(path: &Path, options: &Options) -> Result<Vec<Entry>, LsrError> {
    let mut subdirectories: Vec<std::path::PathBuf> = Vec::new();
    let current = path.to_path_buf();
    let mut entries = Vec::new();

    if path.is_dir() {
        subdirectories.push(current);
    } else {
        return Err(LsrError::InvalidPath(path.to_string_lossy().to_string()));
    }

    while let Some(current) = subdirectories.pop() {
        for std_entry in std::fs::read_dir(&current)? {
            let entry = std_entry?;
            let name = entry.file_name().to_string_lossy().to_string();

            let metadata = entry.metadata()?;
            let size = metadata.len(); // u64
            let modified = DateTime::<Local>::from(metadata.modified()?); // DateTime<Local>
            let readonly = metadata.permissions().readonly(); // bool

            if options.all || !name.starts_with('.') {
                let kind = if metadata.is_dir() {
                    let count = std::fs::read_dir(entry.path())?.count();
                    if options.recursive {
                        subdirectories.push(entry.path());
                    }
                    EntryKind::Directory { count }
                } else {
                    EntryKind::File { size }
                };
                entries.push(Entry::new(name, kind, modified, readonly));
            }
        }
    }

    Ok(entries)
}
