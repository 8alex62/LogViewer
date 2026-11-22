//! Implementations of [`LogParser`](logviewer_application::ports::LogParser),
//! one per supported format (plain text first, then JSON Lines, patterns and
//! access logs).

mod plain;

pub use plain::PlainTextParser;
