use std::io::BufRead;

/// Maximum line length we'll allocate when reading JSONL files.
/// Lines exceeding this are skipped to prevent memory exhaustion from
/// malformed or malicious input. 10 MB is generous for any realistic
/// Claude Code JSONL entry.
pub const MAX_LINE_LEN: usize = 10 * 1024 * 1024;

/// Read a single line from a buffered reader without allocating more than
/// `max_len` bytes. If the line exceeds the limit, the reader is advanced
/// past it and `buf` is cleared (caller sees an empty string).
///
/// Returns the total number of bytes consumed from the reader (including
/// bytes that were skipped for oversized lines), or 0 at EOF.
pub fn read_line_capped(
    reader: &mut impl BufRead,
    buf: &mut String,
    max_len: usize,
) -> std::io::Result<usize> {
    buf.clear();
    let mut total = 0usize;
    let mut oversize = false;
    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            if oversize {
                buf.clear();
            }
            return Ok(total);
        }
        let (chunk, done) = match available.iter().position(|&b| b == b'\n') {
            Some(pos) => (&available[..=pos], true),
            None => (available, false),
        };
        let len = chunk.len();
        total += len;
        if !oversize && total <= max_len {
            match std::str::from_utf8(chunk) {
                Ok(s) => buf.push_str(s),
                Err(_) => oversize = true,
            }
        } else {
            oversize = true;
        }
        reader.consume(len);
        if done {
            if oversize {
                buf.clear();
            }
            return Ok(total);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn reads_normal_line() {
        let data = b"hello world\nsecond\n";
        let mut reader = BufReader::new(&data[..]);
        let mut buf = String::new();
        let n = read_line_capped(&mut reader, &mut buf, 1024).unwrap();
        assert_eq!(n, 12);
        assert_eq!(buf, "hello world\n");
    }

    #[test]
    fn skips_oversized_line() {
        let data = b"short\nthis line is way too long\nok\n";
        let mut reader = BufReader::new(&data[..]);
        let mut buf = String::new();

        // Read first line normally
        read_line_capped(&mut reader, &mut buf, 1024).unwrap();
        assert_eq!(buf, "short\n");

        // Second line exceeds limit of 10 bytes — should be skipped
        let n = read_line_capped(&mut reader, &mut buf, 10).unwrap();
        assert!(n > 0); // bytes were consumed
        assert!(buf.is_empty()); // but buf is cleared

        // Third line is fine
        read_line_capped(&mut reader, &mut buf, 1024).unwrap();
        assert_eq!(buf, "ok\n");
    }

    #[test]
    fn returns_zero_at_eof() {
        let data = b"";
        let mut reader = BufReader::new(&data[..]);
        let mut buf = String::new();
        let n = read_line_capped(&mut reader, &mut buf, 1024).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn handles_no_trailing_newline() {
        let data = b"no newline";
        let mut reader = BufReader::new(&data[..]);
        let mut buf = String::new();
        let n = read_line_capped(&mut reader, &mut buf, 1024).unwrap();
        assert_eq!(n, 10);
        assert_eq!(buf, "no newline");
    }
}
