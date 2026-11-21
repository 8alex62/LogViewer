use logviewer_application::ports::{LogSource, SourceError};

/// In-memory implementation of [`LogSource`].
/// Used for testing and as a fallback when no other source is available.
#[derive(Debug)]
pub struct MemorySource(Vec<u8>);

impl MemorySource {
    /// Creates a new in-memory log source from a vector of bytes.
    /// Useful for testing or when the log is already in memory.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl LogSource for MemorySource {
    fn len(&self) -> u64 {
        self.0.len() as u64
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, SourceError> {
        let Ok(offset) = usize::try_from(offset) else {
            return Ok(0);
        };
        let Some(remaining) = self.0.get(offset..) else {
            return Ok(0);
        };
        let n = remaining.len().min(buf.len());
        buf[..n].copy_from_slice(&remaining[..n]);
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::MemorySource;
    use logviewer_application::ports::{LogSource, SourceError};

    const DATA: &[u8] = b"hello\nworld";

    fn source() -> MemorySource {
        MemorySource::new(DATA.to_vec())
    }

    // ---------- len / is_empty ----------

    #[test]
    fn len_returns_the_number_of_bytes() {
        assert_eq!(source().len(), 11);
    }

    #[test]
    fn empty_source_has_zero_len_and_is_empty() {
        let src = MemorySource::new(Vec::new());
        assert_eq!(src.len(), 0);
        assert!(src.is_empty());
    }

    #[test]
    fn non_empty_source_is_not_empty() {
        assert!(!source().is_empty());
    }

    // ---------- regular reads ----------

    #[test]
    fn reads_everything_when_the_buffer_is_exactly_the_right_size() -> Result<(), SourceError> {
        let mut buf = [0u8; 11];
        let n = source().read_at(0, &mut buf)?;
        assert_eq!(n, 11);
        assert_eq!(&buf, DATA);
        Ok(())
    }

    #[test]
    fn fills_only_the_start_of_a_larger_buffer() -> Result<(), SourceError> {
        let mut buf = [0xAAu8; 20];
        let n = source().read_at(0, &mut buf)?;
        assert_eq!(n, 11);
        assert_eq!(&buf[..n], DATA);
        assert!(
            buf[n..].iter().all(|&b| b == 0xAA),
            "the rest of the buffer must stay untouched"
        );
        Ok(())
    }

    #[test]
    fn reads_at_most_the_buffer_size() -> Result<(), SourceError> {
        let mut buf = [0u8; 4];
        let n = source().read_at(0, &mut buf)?;
        assert_eq!(n, 4);
        assert_eq!(&buf, b"hell");
        Ok(())
    }

    #[test]
    fn reads_from_the_middle() -> Result<(), SourceError> {
        let mut buf = [0u8; 5];
        let n = source().read_at(6, &mut buf)?;
        assert_eq!(n, 5);
        assert_eq!(&buf, b"world");
        Ok(())
    }

    #[test]
    fn returns_a_short_read_near_the_end() -> Result<(), SourceError> {
        let mut buf = [0u8; 8];
        let n = source().read_at(8, &mut buf)?; // only "rld" is left
        assert_eq!(n, 3);
        assert_eq!(&buf[..n], b"rld");
        Ok(())
    }

    // ---------- end of source and edge cases ----------

    #[test]
    fn returns_zero_at_exactly_the_end() -> Result<(), SourceError> {
        let mut buf = [0u8; 4];
        assert_eq!(source().read_at(11, &mut buf)?, 0);
        Ok(())
    }

    #[test]
    fn returns_zero_past_the_end() -> Result<(), SourceError> {
        let mut buf = [0u8; 4];
        assert_eq!(source().read_at(12, &mut buf)?, 0);
        assert_eq!(
            source().read_at(u64::MAX, &mut buf)?,
            0,
            "must not overflow nor panic"
        );
        Ok(())
    }

    #[test]
    fn empty_buffer_reads_nothing() -> Result<(), SourceError> {
        let mut buf = [0u8; 0];
        assert_eq!(source().read_at(0, &mut buf)?, 0);
        Ok(())
    }

    #[test]
    fn empty_source_reads_nothing() -> Result<(), SourceError> {
        let mut buf = [0u8; 4];
        assert_eq!(MemorySource::new(Vec::new()).read_at(0, &mut buf)?, 0);
        Ok(())
    }

    // ---------- positional behavior ----------

    #[test]
    fn reading_does_not_move_any_cursor() -> Result<(), SourceError> {
        let src = source();
        let (mut a, mut b) = ([0u8; 5], [0u8; 5]);
        src.read_at(0, &mut a)?;
        src.read_at(0, &mut b)?;
        assert_eq!(a, b, "two reads at the same offset return the same bytes");
        Ok(())
    }

    #[test]
    fn chunked_reads_rebuild_the_whole_content() -> Result<(), SourceError> {
        // Mimics the future line index loop: read by chunks until 0 is returned.
        let src = source();
        let mut out = Vec::new();
        let mut buf = [0u8; 3];
        let mut offset = 0u64;
        loop {
            let n = src.read_at(offset, &mut buf)?;
            if n == 0 {
                break;
            }
            out.extend_from_slice(&buf[..n]);
            offset += n as u64;
        }
        assert_eq!(out, DATA);
        Ok(())
    }
}
