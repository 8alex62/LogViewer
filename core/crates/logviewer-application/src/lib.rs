//! `LogViewer` application layer.
//!
//! Hosts the use cases (open a file, get lines, filter, search, tail) and the
//! ports they need, expressed as traits. Concrete implementations live in
//! `logviewer-infrastructure`; the presentation crates wire them together.

pub mod ports;
pub mod use_cases;
