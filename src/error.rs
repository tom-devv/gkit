use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("git error: {0}")]
    GitError(#[from] git2::Error),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("kit error: {0}")]
    KitError(String),

    #[error("ratatui error: {0}")]
    RatatuiError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
