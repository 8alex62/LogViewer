//! Use cases, one module per user action: `OpenFile`, `GetLines`, `Filter`,
//! `Search`, `Tail`. Each one depends on the ports only, never on a concrete
//! platform type.
//!
//! The first ones (`OpenFile`, `GetLines`) arrive with the US "Read a plain
//! text file and return lines by range".

mod get_lines;

pub use get_lines::{GetLinesError, LinesPage, get_lines};
