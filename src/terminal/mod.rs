use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::errors::RuneError;

pub struct RawModeGuard {}

impl RawModeGuard {
    pub fn new() -> Result<Self, RuneError> {
        // Enable raw mode on terminal
        let guard = Self {};
        guard.enable_raw_mode()?;
        Ok(guard)
    }

    fn enable_raw_mode(&self) -> Result<(), RuneError> {
        // Code to enable raw mode
        enable_raw_mode().map_err(|e| RuneError::TerminalError(e.to_string()))
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // Disable raw mode on terminal
        if let Err(e) = disable_raw_mode() {
            eprintln!("Failed to disable raw mode: {}", e);
        }
    }
}
