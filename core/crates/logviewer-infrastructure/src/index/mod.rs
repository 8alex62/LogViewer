use logviewer_application::ports::{LogSource, SourceError};
use std::ops::Range;

/// Size of the blocks read while scanning the source.
const CHUNK_SIZE: usize = 64 * 1024;

/// Positions of the line terminators of a source, built incrementally.
#[derive(Debug, Default)]
pub struct LineIndex {
    /// Offset just after each `\n` (exclusive end of each complete line).
    line_ends: Vec<u64>,
    /// How far the source has been scanned.
    scanned_up_to: u64,
}

impl LineIndex {
    /// Scans the source from where the previous call stopped and returns
    /// the number of lines completed by this call.
    pub fn extend(&mut self, source: &dyn LogSource) -> Result<u64, SourceError> {
        let mut buf = vec![0u8; CHUNK_SIZE];
        let mut lines_found = 0;

        loop {
            let n = source.read_at(self.scanned_up_to, &mut buf)?;
            if n == 0 {
                break;
            }
            for (i, &b) in buf[..n].iter().enumerate() {
                if b == b'\n' {
                    self.line_ends.push(self.scanned_up_to + i as u64 + 1);
                    lines_found += 1;
                }
            }
            self.scanned_up_to += n as u64;
        }
        Ok(lines_found)
    }

    /// Number of lines, including a last line without a trailing `\n`.
    pub fn line_count(&self) -> u64 {
        self.line_ends.len() as u64 + u64::from(self.has_pending_line())
    }

    /// Whether bytes follow the last `\n` (a line still being written when tailing).
    pub fn has_pending_line(&self) -> bool {
        self.line_ends.last().copied().unwrap_or(0) < self.scanned_up_to
    }

    /// Byte range of a line, without its `\n` (a `\r` before it is kept).
    pub fn line_range(&self, line: u64) -> Option<Range<u64>> {
        let i = usize::try_from(line).ok()?;
        let start = match i.checked_sub(1) {
            None => 0,
            Some(previous) => *self.line_ends.get(previous)?,
        };
        let end = match self.line_ends.get(i) {
            Some(&after_newline) => after_newline - 1,
            None if start < self.scanned_up_to => self.scanned_up_to,
            None => return None,
        };
        Some(start..end)
    }
}

#[cfg(test)]
mod tests {
    use super::{CHUNK_SIZE, LineIndex};
    use crate::reader::memory::MemorySource;
    use logviewer_application::ports::SourceError;

    fn index_of(bytes: &[u8]) -> Result<LineIndex, SourceError> {
        let mut index = LineIndex::default();
        index.extend(&MemorySource::new(bytes.to_vec()))?;
        Ok(index)
    }

    // ---------- line endings ----------

    #[test]
    fn empty_source_has_no_lines() -> Result<(), SourceError> {
        let index = index_of(b"")?;
        assert_eq!(index.line_count(), 0);
        assert!(!index.has_pending_line());
        assert_eq!(index.line_range(0), None);
        Ok(())
    }

    #[test]
    fn lf_lines_with_trailing_newline() -> Result<(), SourceError> {
        let index = index_of(b"ab\ncd\n")?;
        assert_eq!(index.line_count(), 2);
        assert_eq!(index.line_range(0), Some(0..2));
        assert_eq!(index.line_range(1), Some(3..5));
        assert!(!index.has_pending_line());
        Ok(())
    }

    #[test]
    fn last_line_without_trailing_newline_is_counted() -> Result<(), SourceError> {
        let index = index_of(b"ab\ncd")?;
        assert_eq!(index.line_count(), 2);
        assert_eq!(index.line_range(1), Some(3..5));
        assert!(index.has_pending_line());
        Ok(())
    }

    #[test]
    fn crlf_lines_keep_the_cr_in_their_range() -> Result<(), SourceError> {
        let index = index_of(b"ab\r\ncd\r\n")?;
        assert_eq!(index.line_count(), 2);
        assert_eq!(index.line_range(0), Some(0..3)); // "ab\r", stripped on extraction
        assert_eq!(index.line_range(1), Some(4..7));
        Ok(())
    }

    #[test]
    fn empty_lines_are_lines() -> Result<(), SourceError> {
        let index = index_of(b"\n\nx")?;
        assert_eq!(index.line_count(), 3);
        assert_eq!(index.line_range(0), Some(0..0));
        assert_eq!(index.line_range(1), Some(1..1));
        assert_eq!(index.line_range(2), Some(2..3));
        Ok(())
    }

    #[test]
    fn line_out_of_range_is_none() -> Result<(), SourceError> {
        let index = index_of(b"a\nb\n")?;
        assert_eq!(index.line_range(2), None);
        assert_eq!(index.line_range(u64::MAX), None);
        Ok(())
    }

    // ---------- chunk boundaries ----------

    #[test]
    fn line_longer_than_a_chunk() -> Result<(), SourceError> {
        let mut data = vec![b'x'; 3 * CHUNK_SIZE];
        data.extend_from_slice(b"\nend");
        let index = index_of(&data)?;
        assert_eq!(index.line_count(), 2);
        assert_eq!(index.line_range(0), Some(0..3 * CHUNK_SIZE as u64));
        Ok(())
    }

    #[test]
    fn crlf_split_across_two_chunks() -> Result<(), SourceError> {
        // '\r' is the last byte of the first chunk, '\n' the first of the second.
        let mut data = vec![b'x'; CHUNK_SIZE - 1];
        data.extend_from_slice(b"\r\nnext\n");
        let index = index_of(&data)?;
        assert_eq!(index.line_count(), 2);
        assert_eq!(index.line_range(0), Some(0..CHUNK_SIZE as u64)); // ends with '\r'
        Ok(())
    }

    #[test]
    fn newline_as_the_last_byte_of_a_chunk() -> Result<(), SourceError> {
        let mut data = vec![b'x'; CHUNK_SIZE - 1];
        data.extend_from_slice(b"\ny");
        let index = index_of(&data)?;
        assert_eq!(index.line_count(), 2);
        assert_eq!(
            index.line_range(1),
            Some(CHUNK_SIZE as u64..CHUNK_SIZE as u64 + 1)
        );
        Ok(())
    }

    // ---------- incremental scan (tail-ready) ----------

    #[test]
    fn extend_returns_the_number_of_completed_lines() -> Result<(), SourceError> {
        let mut index = LineIndex::default();
        assert_eq!(index.extend(&MemorySource::new(b"a\nb\nc".to_vec()))?, 2);
        Ok(())
    }

    #[test]
    fn extend_picks_up_appended_bytes() -> Result<(), SourceError> {
        // Same file read twice: the second snapshot has grown.
        let mut index = LineIndex::default();
        index.extend(&MemorySource::new(b"a\nb".to_vec()))?;
        assert_eq!(index.line_count(), 2);
        assert!(index.has_pending_line());

        let completed = index.extend(&MemorySource::new(b"a\nb\nc\n".to_vec()))?;
        assert_eq!(completed, 2, "the pending \"b\" and the new \"c\"");
        assert_eq!(index.line_count(), 3);
        assert!(!index.has_pending_line());
        assert_eq!(index.line_range(1), Some(2..3));
        Ok(())
    }

    #[test]
    fn extend_without_new_bytes_changes_nothing() -> Result<(), SourceError> {
        let source = MemorySource::new(b"a\nb\n".to_vec());
        let mut index = LineIndex::default();
        index.extend(&source)?;
        assert_eq!(index.extend(&source)?, 0);
        assert_eq!(index.line_count(), 2);
        Ok(())
    }
}
