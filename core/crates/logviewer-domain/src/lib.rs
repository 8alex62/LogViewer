//! `LogViewer` domain layer.
//!
//! Pure business types shared by every other layer. This crate has no
//! dependencies and knows nothing about files, transports or platforms.

mod entry;
mod filter;
mod level;

pub use entry::{LogEntry, LogFormat};
pub use filter::Filter;
pub use level::LogLevel;
