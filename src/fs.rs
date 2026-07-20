pub(crate) struct Fs {}

impl Fs {
    pub(crate) fn new() -> Self {
        Fs {}
    }
}

#[cfg(test)]
use super::*;

#[test]
fn test_fs_creation() {
    let fs = Fs::new();
}
