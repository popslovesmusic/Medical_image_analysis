//! Defines the canonical `DreamError` type for the Chromatic Core.
//!
//! All fallible operations within this crate must return a `Result` wrapping
//! this error type, as mandated by the `AGENTS.md` ZAG constraint [I.B.3].

use std::fmt;

/// The canonical error type for all Chromatic Core operations.
#[derive(Debug)]
pub enum DreamError {
    /// Error originating from the tensor module (e.g., shape mismatch, invalid ops).
    Tensor(String),

    /// Error originating from the bridge module (e.g., failed color/spectral conversion).
    Bridge(String),

    /// Error originating from the dream module (e.g., pool failure, invalid seed).
    Dream(String),

    /// Error originating from the diagnostics module.
    Diagnostics(String),

    /// Error related to I/O operations (e.g., loading configs, LUTs, or checkpoints).
    Io(std::io::Error),

    /// Error related to parsing configuration files (e.g., TOML, JSON).
    Config(String),

    /// A critical validation failure (e.g., determinism check failed, hash mismatch).
    Validation(String),
}

impl fmt::Display for DreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DreamError::Tensor(s) => write!(f, "Tensor Error: {}", s),
            DreamError::Bridge(s) => write!(f, "Bridge Error: {}", s),
            DreamError::Dream(s) => write!(f, "Dream Error: {}", s),
            DreamError::Diagnostics(s) => write!(f, "Diagnostics Error: {}", s),
            DreamError::Io(e) => write!(f, "IO Error: {}", e),
            DreamError::Config(s) => write!(f, "Configuration Error: {}", s),
            DreamError::Validation(s) => write!(f, "Validation Failure: {}", s),
        }
    }
}

impl std::error::Error for DreamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DreamError::Io(e) => Some(e),
            _ => None,
        }
    }
}

/// Convenience `From` implementation for standard I/O errors.
impl From<std::io::Error> for DreamError {
    fn from(e: std::io::Error) -> Self {
        DreamError::Io(e)
    }
}

/// A convenience `Result` type alias using the canonical `DreamError`.
pub type CoreResult<T> = Result<T, DreamError>;