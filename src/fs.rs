use crate::entry::Entry;
use chrono::{DateTime, Local};
use std::path::Path;

pub fn list(path: &Path, all: bool) -> Result<Vec<Entry>, std::io::Error> {
    if path.is_dir() {
        let mut entries = Vec::new();
        for std_entry in std::fs::read_dir(path)? {
            let entry = std_entry?;
            let name = entry.file_name().to_string_lossy().to_string();

            let metadata = entry.metadata()?;
            let size = metadata.len(); // u64
            let modified = DateTime::<Local>::from(metadata.modified()?); // DateTime<Local>
            let readonly = metadata.permissions().readonly(); // bool

            if all || !name.starts_with('.') {
                entries.push(Entry::new(name, size, modified, readonly));
            }
        }
        Ok(entries)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Fichier spécifié",
        ))
    }
}
