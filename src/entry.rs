use chrono::{DateTime, Local};

pub enum EntryKind {
    File { size: u64 },
    Directory { count: usize },
}

impl EntryKind {
    pub fn size(&self) -> String {
        match self {
            EntryKind::File { size } => format!("{} bytes", size),
            EntryKind::Directory { count } => format!("{} entries", count),
        }
    }
}

impl std::fmt::Display for EntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryKind::File { .. } => write!(f, "file"),
            EntryKind::Directory { .. } => write!(f, "dir"),
        }
    }
}

pub struct Entry {
    name: String,
    kind: EntryKind,
    modified: DateTime<Local>,
    readonly: bool,
}

impl Entry {
    pub fn new(name: String, kind: EntryKind, modified: DateTime<Local>, readonly: bool) -> Self {
        Entry {
            name,
            kind,
            modified,
            readonly,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &EntryKind {
        &self.kind
    }

    pub fn modified(&self) -> DateTime<Local> {
        self.modified
    }

    pub fn readonly(&self) -> bool {
        self.readonly
    }

    pub fn format_entry(&self, long: bool) -> String {
        if long {
            format!(
                "{} \t{:<30} \t{:<10} \t{:<30} \t{:<20} \t{:<10}",
                match self.kind() {
                    EntryKind::File { .. } => "",
                    EntryKind::Directory { .. } => ":: ",
                },
                self.name(),
                self.kind().to_string(),
                self.kind().size(),
                self.modified().format("%Y-%m-%d %H:%M").to_string(),
                self.readonly()
            )
        } else {
            self.name().to_string()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_entry_name() {
        let entry = Entry::new(
            String::from("foo.txt"),
            EntryKind::File { size: 1024 },
            chrono::Local::now(),
            false,
        );
        assert_eq!(entry.name(), "foo.txt");
    }

    #[test]
    fn test_format_entry_short() {
        let entry = Entry::new(
            String::from("foo.txt"),
            EntryKind::File { size: 1024 },
            chrono::Local::now(),
            false,
        );
        assert_eq!(entry.format_entry(false), "foo.txt");
    }

    #[test]
    fn test_format_entry_long() {
        let entry = Entry::new(
            String::from("foo.txt"),
            EntryKind::File { size: 1024 },
            chrono::Local::now(),
            false,
        );
        let result = entry.format_entry(true);
        assert!(result.contains("foo.txt"));
        assert!(result.contains("file"));
        assert!(result.contains("false"));
    }

    #[test]
    fn test_entry_kind_display() {
        let file_kind = EntryKind::File { size: 1024 };
        let dir_kind = EntryKind::Directory { count: 5 };

        assert_eq!(file_kind.to_string(), "file");
        assert_eq!(dir_kind.to_string(), "dir");
    }

    #[test]
    fn test_entry_kind_size() {
        let file_kind = EntryKind::File { size: 1024 };
        let dir_kind = EntryKind::Directory { count: 5 };

        assert_eq!(file_kind.size(), "1024 bytes");
        assert_eq!(dir_kind.size(), "5 entries");
    }
}
