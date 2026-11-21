use crate::LogLevel;

/// Criteria applied to entries before they are returned to the UI.
///
/// An empty filter (the default) keeps every entry. Date range and text/regex
/// criteria are added with the filtering stories (v0.3).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Filter {
    /// Levels to keep; empty means every level.
    pub levels: Vec<LogLevel>,
}
