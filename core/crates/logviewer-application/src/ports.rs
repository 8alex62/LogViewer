//! Ports: what the use cases need from the outside world.
//!
//! The application layer only depends on these traits. The infrastructure
//! provides one implementation per platform (memory-mapped file on desktop,
//! `Blob.slice` in the browser), selected by the presentation crate.

use logviewer_domain::LogEntry;

/// Random-access, read-only source of bytes (a log file).
///
/// The whole file is never loaded in memory: callers read ranges on demand.
pub trait LogSource {
    /// Total size of the source, in bytes.
    fn len(&self) -> u64;

    /// Whether the source holds no bytes.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reads up to `buf.len()` bytes starting at `offset` and returns how many
    /// were read (0 at the end of the source).
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, SourceError>;
}

/// Turns the raw text of an entry into a structured [`LogEntry`].
pub trait LogParser {
    fn parse(&self, line_number: u64, raw: &str) -> LogEntry;
}

/// Errors raised by a [`LogSource`].
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("file not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("I/O error: {0}")]
    Io(String),
}
