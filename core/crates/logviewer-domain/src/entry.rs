use crate::LogLevel;

/// Log file format, by priority of support (see the specs, §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LogFormat {
    /// Fallback: one line is one entry, no structured fields.
    #[default]
    PlainText,
    /// JSON Lines (Serilog, `NLog` JSON, bunyan…).
    JsonLines,
    /// Pattern-based layouts (log4j, log4net, `NLog`, syslog).
    Pattern,
    /// Web access logs (Apache / Nginx combined).
    AccessLog,
}

/// One log entry, as displayed by the UI.
///
/// Fields are filled progressively by the parsers: a plain text entry only has
/// its line number and raw text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    /// Zero-based index of the first line of the entry in the file.
    pub line_number: u64,
    /// Raw text of the entry, without the line terminator.
    pub raw: String,
    /// Severity, when the format exposes one.
    pub level: Option<LogLevel>,
}
