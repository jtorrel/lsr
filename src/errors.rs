#[derive(Debug, thiserror::Error)]
pub enum LsrError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
