use logviewer_application::ports::LogParser;
use logviewer_domain::LogEntry;

/// Fallback parser: one line is one entry, without any structured field.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlainTextParser;

impl LogParser for PlainTextParser {
    fn parse(&self, line_number: u64, raw: &str) -> LogEntry {
        LogEntry {
            line_number,
            raw: raw.to_owned(),
            level: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlainTextParser;
    use logviewer_application::ports::LogParser;
    use logviewer_domain::LogEntry;

    fn parse(raw: &str) -> LogEntry {
        PlainTextParser.parse(0, raw)
    }

    #[test]
    fn builds_an_entry_from_a_line() {
        let expected = LogEntry {
            line_number: 7,
            raw: "hello world".to_owned(),
            level: None,
        };
        assert_eq!(PlainTextParser.parse(7, "hello world"), expected);
    }

    #[test]
    fn accepts_line_numbers_beyond_32_bits() {
        let line = u64::from(u32::MAX) + 1;
        assert_eq!(PlainTextParser.parse(line, "x").line_number, line);
    }

    #[test]
    fn never_guesses_a_level() {
        assert_eq!(parse("2026-11-15 ERROR connection refused").level, None);
    }

    #[test]
    fn does_not_interpret_json() {
        let json = r#"{"level":"error","message":"boom"}"#;
        let entry = parse(json);
        assert_eq!(entry.raw, json);
        assert_eq!(entry.level, None);
    }

    #[test]
    fn empty_line_is_a_valid_entry() {
        assert_eq!(parse("").raw, "");
    }

    #[test]
    fn keeps_leading_and_trailing_whitespace() {
        assert_eq!(parse("\t  at App.Main()  ").raw, "\t  at App.Main()  ");
    }

    #[test]
    fn keeps_non_ascii_text() {
        assert_eq!(parse("démarrage ✅ 日本語").raw, "démarrage ✅ 日本語");
    }

    #[test]
    fn entry_outlives_the_read_buffer() {
        let buffer = String::from("temporary");
        let entry = parse(&buffer);
        drop(buffer);
        assert_eq!(entry.raw, "temporary");
    }

    #[test]
    fn works_through_the_port() {
        let parser: &dyn LogParser = &PlainTextParser;
        assert_eq!(parser.parse(1, "x").raw, "x");
    }
}
