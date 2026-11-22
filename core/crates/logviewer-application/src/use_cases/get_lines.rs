use std::ops::Range;

use logviewer_domain::LogEntry;

use crate::ports::{LineIndexer, LogParser, LogSource, SourceError};

/// A page of entries and the total number of lines of the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinesPage {
    /// Number of lines of the whole file, not of the page.
    pub total_lines: u64,
    /// Entries of the requested range, in file order.
    pub entries: Vec<LogEntry>,
}

/// Errors raised by [`get_lines`].
#[derive(Debug, thiserror::Error)]
pub enum GetLinesError {
    #[error("line {start} is out of range: the file has {total} lines")]
    OutOfRange { start: u64, total: u64 },
    #[error(transparent)]
    Source(#[from] SourceError),
}

/// Returns the entries of the lines `[start, start + count)`.
///
/// The page is cut at the end of the file. `start` equal to the line count
/// returns an empty page; a larger `start` is an error.
pub fn get_lines(
    source: &dyn LogSource,
    index: &dyn LineIndexer,
    parser: &dyn LogParser,
    start: u64,
    count: u32,
) -> Result<LinesPage, GetLinesError> {
    let total = index.line_count();
    if start > total {
        return Err(GetLinesError::OutOfRange { start, total });
    }

    let end = start.saturating_add(u64::from(count)).min(total);
    let mut entries = Vec::new();
    for line in start..end {
        let Some(range) = index.locate(source, line)? else {
            break;
        };
        let bytes = read_range(source, range)?;
        let text = String::from_utf8_lossy(&bytes);
        let text = text.strip_suffix('\r').unwrap_or(&text);
        entries.push(parser.parse(line, text));
    }

    Ok(LinesPage {
        total_lines: total,
        entries,
    })
}

/// Reads every byte of `range`, looping because `read_at` may return less.
fn read_range(source: &dyn LogSource, range: Range<u64>) -> Result<Vec<u8>, SourceError> {
    let len = usize::try_from(range.end - range.start)
        .map_err(|_| SourceError::Io("line too long for this platform".to_owned()))?;

    let mut buf = vec![0u8; len];
    let mut filled = 0;

    while filled < len {
        let n = source.read_at(range.start + filled as u64, &mut buf[filled..])?;
        if n == 0 {
            break;
        }
        filled += n;
    }

    buf.truncate(filled);
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use logviewer_domain::LogEntry;

    use super::{GetLinesError, get_lines};
    use crate::ports::{LineIndexer, LogParser, LogSource, SourceError};

    // ---------- test doubles (the application cannot use the infrastructure) ----------

    /// Bytes in memory, returning at most `max_read` bytes per call.
    struct FakeSource {
        bytes: Vec<u8>,
        max_read: usize,
    }

    impl LogSource for FakeSource {
        fn len(&self) -> u64 {
            self.bytes.len() as u64
        }

        fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, SourceError> {
            let Ok(offset) = usize::try_from(offset) else {
                return Ok(0);
            };
            let Some(rest) = self.bytes.get(offset..) else {
                return Ok(0);
            };
            let n = rest.len().min(buf.len()).min(self.max_read);
            buf[..n].copy_from_slice(&rest[..n]);
            Ok(n)
        }
    }

    /// A source whose reads always fail.
    struct BrokenSource;

    impl LogSource for BrokenSource {
        fn len(&self) -> u64 {
            10
        }

        fn read_at(&self, _offset: u64, _buf: &mut [u8]) -> Result<usize, SourceError> {
            Err(SourceError::Io("disk unplugged".to_owned()))
        }
    }

    /// Line ranges computed in the simplest way, to test `get_lines` alone.
    struct FakeIndex(Vec<Range<u64>>);

    impl FakeIndex {
        fn of(bytes: &[u8]) -> Self {
            let mut ranges = Vec::new();
            let mut start = 0;
            for (i, &b) in bytes.iter().enumerate() {
                if b == b'\n' {
                    ranges.push(start..i as u64);
                    start = i as u64 + 1;
                }
            }
            if start < bytes.len() as u64 {
                ranges.push(start..bytes.len() as u64);
            }
            Self(ranges)
        }
    }

    impl LineIndexer for FakeIndex {
        fn extend(&mut self, _source: &dyn LogSource) -> Result<u64, SourceError> {
            Ok(0)
        }

        fn line_count(&self) -> u64 {
            self.0.len() as u64
        }

        fn locate(
            &self,
            _source: &dyn LogSource,
            line: u64,
        ) -> Result<Option<Range<u64>>, SourceError> {
            Ok(usize::try_from(line)
                .ok()
                .and_then(|i| self.0.get(i).cloned()))
        }
    }

    /// Copies the text as is, like the plain text parser.
    struct EchoParser;

    impl LogParser for EchoParser {
        fn parse(&self, line_number: u64, raw: &str) -> LogEntry {
            LogEntry {
                line_number,
                raw: raw.to_owned(),
                level: None,
            }
        }
    }

    fn source_of(bytes: &[u8]) -> FakeSource {
        FakeSource {
            bytes: bytes.to_vec(),
            max_read: usize::MAX,
        }
    }

    /// Runs `get_lines` on `bytes` and returns the texts of the page.
    fn texts(bytes: &[u8], start: u64, count: u32) -> Result<Vec<String>, GetLinesError> {
        let page = get_lines(
            &source_of(bytes),
            &FakeIndex::of(bytes),
            &EchoParser,
            start,
            count,
        )?;
        Ok(page.entries.into_iter().map(|e| e.raw).collect())
    }

    const FILE: &[u8] = b"zero\none\ntwo\nthree\nfour\n";

    // ---------- paging ----------

    #[test]
    fn returns_the_requested_page() -> Result<(), GetLinesError> {
        assert_eq!(texts(FILE, 1, 2)?, ["one", "two"]);
        Ok(())
    }

    #[test]
    fn total_lines_counts_the_whole_file() -> Result<(), GetLinesError> {
        let page = get_lines(&source_of(FILE), &FakeIndex::of(FILE), &EchoParser, 1, 2)?;
        assert_eq!(page.total_lines, 5);
        assert_eq!(page.entries.len(), 2);
        Ok(())
    }

    #[test]
    fn entries_keep_their_line_number_in_the_file() -> Result<(), GetLinesError> {
        let page = get_lines(&source_of(FILE), &FakeIndex::of(FILE), &EchoParser, 3, 2)?;
        let numbers: Vec<u64> = page.entries.iter().map(|e| e.line_number).collect();
        assert_eq!(numbers, [3, 4]);
        Ok(())
    }

    #[test]
    fn last_page_is_cut_at_the_end_of_the_file() -> Result<(), GetLinesError> {
        assert_eq!(texts(FILE, 3, 100)?, ["three", "four"]);
        assert_eq!(texts(FILE, 4, u32::MAX)?, ["four"]);
        Ok(())
    }

    #[test]
    fn count_zero_returns_no_entries() -> Result<(), GetLinesError> {
        assert_eq!(texts(FILE, 2, 0)?, Vec::<String>::new());
        Ok(())
    }

    // ---------- out of range ----------

    #[test]
    fn start_at_the_line_count_returns_an_empty_page() -> Result<(), GetLinesError> {
        assert_eq!(texts(FILE, 5, 10)?, Vec::<String>::new());
        assert_eq!(texts(b"", 0, 10)?, Vec::<String>::new(), "empty file");
        Ok(())
    }

    #[test]
    fn start_past_the_line_count_is_an_error() {
        let result = texts(FILE, 6, 10);
        assert!(
            matches!(
                result,
                Err(GetLinesError::OutOfRange { start: 6, total: 5 })
            ),
            "got {result:?}"
        );
    }

    #[test]
    fn huge_start_is_an_error_not_a_panic() {
        let result = texts(FILE, u64::MAX, u32::MAX);
        assert!(
            matches!(result, Err(GetLinesError::OutOfRange { .. })),
            "got {result:?}"
        );
    }

    // ---------- text extraction ----------

    #[test]
    fn crlf_terminators_are_removed() -> Result<(), GetLinesError> {
        assert_eq!(texts(b"a\r\nb\r\n", 0, 10)?, ["a", "b"]);
        Ok(())
    }

    #[test]
    fn only_one_trailing_cr_is_removed() -> Result<(), GetLinesError> {
        assert_eq!(texts(b"a\rb\r\r\n", 0, 10)?, ["a\rb\r"]);
        Ok(())
    }

    #[test]
    fn last_line_without_newline_is_returned() -> Result<(), GetLinesError> {
        assert_eq!(texts(b"a\nlast", 0, 10)?, ["a", "last"]);
        Ok(())
    }

    #[test]
    fn whitespace_and_empty_lines_are_kept() -> Result<(), GetLinesError> {
        assert_eq!(texts(b"\n  indented  \n", 0, 10)?, ["", "  indented  "]);
        Ok(())
    }

    #[test]
    fn invalid_utf8_is_replaced_not_an_error() -> Result<(), GetLinesError> {
        assert_eq!(texts(b"caf\xE9\nok", 0, 10)?, ["caf\u{FFFD}", "ok"]);
        Ok(())
    }

    // ---------- reading ----------

    #[test]
    fn reads_a_line_across_several_short_reads() -> Result<(), GetLinesError> {
        let bytes = b"first line\nsecond line\n";
        let source = FakeSource {
            bytes: bytes.to_vec(),
            max_read: 3,
        };
        let page = get_lines(&source, &FakeIndex::of(bytes), &EchoParser, 0, 10)?;
        let texts: Vec<&str> = page.entries.iter().map(|e| e.raw.as_str()).collect();
        assert_eq!(texts, ["first line", "second line"]);
        Ok(())
    }

    #[test]
    fn source_errors_are_propagated() {
        let index = FakeIndex::of(b"boom");
        let result = get_lines(&BrokenSource, &index, &EchoParser, 0, 1);
        assert!(
            matches!(result, Err(GetLinesError::Source(SourceError::Io(_)))),
            "got {result:?}"
        );
    }
}
