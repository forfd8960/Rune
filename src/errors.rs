use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuneError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Terminal error: {0}")]
    TerminalError(String),
    #[error("Editor error: {0}")]
    EditorError(String),
}
