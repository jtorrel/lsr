use chrono::{DateTime, Local};

pub struct Entry {
    name: String,
    size: u64,
    modified: DateTime<Local>,
    readonly: bool,
}

impl Entry {
    pub fn new(name: String, size: u64, modified: DateTime<Local>, readonly: bool) -> Self {
        Entry {
            name,
            size,
            modified,
            readonly,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
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
                "{} {} {:?} {}",
                self.name(),
                self.size(),
                self.modified(),
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
        let entry = Entry::new(String::from("foo.txt"), 0, chrono::Local::now(), false);
        assert_eq!(entry.name(), "foo.txt");
    }

    #[test]
    fn test_format_entry_short() {
        let entry = Entry::new(String::from("foo.txt"), 1024, chrono::Local::now(), false);
        assert_eq!(entry.format_entry(false), "foo.txt");
    }

    #[test]
    fn test_format_entry_long() {
        let entry = Entry::new(String::from("foo.txt"), 1024, chrono::Local::now(), false);
        let result = entry.format_entry(true);
        assert!(result.contains("foo.txt"));
        assert!(result.contains("1024"));
        assert!(result.contains("false"));
    }
}
