//! Ports: what the use cases need from the outside world.
//!
//! The application layer only depends on these traits. The infrastructure
//! provides one implementation per platform (memory-mapped file on desktop,
//! `Blob.slice` in the browser), selected by the presentation crate.

use logviewer_domain::LogEntry;
use std::ops::Range;

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

/// Maps line numbers to byte ranges of a [`LogSource`].
///
/// An indexer follows a single source: always pass it the same one.
pub trait LineIndexer {
    /// Scans the bytes added since the previous call and returns the number
    /// of lines completed by this call.
    fn extend(&mut self, source: &dyn LogSource) -> Result<u64, SourceError>;

    /// Number of lines, including a last line without a trailing `\n`.
    fn line_count(&self) -> u64;

    /// Byte range of a line without its `\n`, or `None` past the last line.
    ///
    /// The source is provided because a sparse index has to read it.
    fn locate(&self, source: &dyn LogSource, line: u64) -> Result<Option<Range<u64>>, SourceError>;
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
