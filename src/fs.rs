use crate::entry::Entry;
use std::path::Path;

pub fn list(path: &Path, all: bool) -> Result<Vec<Entry>, std::io::Error> {
    if path.is_dir() {
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if all || !name.starts_with('.') {
                entries.push(Entry::new(name));
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
