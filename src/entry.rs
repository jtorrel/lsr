pub struct Entry {
    name: String,
}

impl Entry {
    pub fn new(name: String) -> Self {
        Entry { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_name() {
        let entry = Entry::new(String::from("foo.txt"));
        assert_eq!(entry.name(), "foo.txt");
    }
}
