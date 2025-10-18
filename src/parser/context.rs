//! Line-based parsing context: normalizes newlines, indexes lines once, and
//! provides efficient, zero-allocation access to lines with lookahead.

#[derive(Debug)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Context {
    buf: String,
    pub line_range: Vec<Range>,
    /// Index of the first unconsumed line
    pub cur_line_position: usize,
}

impl Context {
    /// Create a context from source text.
    ///
    /// Normalizes newline sequences (CRLF and CR -> LF) and pre-indexes line
    /// ranges so that lookups are O(1) and return borrowed `&str` slices.
    pub(crate) fn new(src: &str) -> Self {
        let buf = normalize_newlines(src);
        let line_range = index_lines(&buf);
        Self {
            buf,
            line_range,
            cur_line_position: 0,
        }
    }

    /// Total number of lines indexed.
    #[inline]
    fn line_count(&self) -> usize {
        self.line_range.len()
    }

    /// Returns the current line without the trailing newline, if any.
    ///
    /// Returns `None` at EOF.
    #[inline]
    pub fn peek_line(&self) -> Option<&str> {
        self.peek_line_n(0)
    }

    /// Returns the nth lookahead line (0 = current), or `None` if out of bounds.
    #[inline]
    pub fn peek_line_n(&self, n: usize) -> Option<&str> {
        let j = self.cur_line_position.checked_add(n)?;
        let span = self.line_range.get(j)?;
        // Slice the line, trimming a single trailing '\n' if present so callers
        // see line content without the newline.
        let mut end = span.end;
        if end > span.start && self.buf.as_bytes()[end - 1] == b'\n' {
            end -= 1;
        }
        Some(&self.buf[span.start..end])
    }

    /// True if there are no more lines to consume.
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.cur_line_position >= self.line_count()
    }

    /// Advance to the next line (no-op at EOF).
    #[inline]
    pub fn advance(&mut self) {
        if !self.is_eof() {
            self.cur_line_position += 1;
        }
    }

    /// Returns the current line, or an empty string at EOF.
    ///
    /// This makes it convenient to call `is_blank(ctx.current_line())` without
    /// checking for `None`.
    #[inline]
    pub fn current_line(&self) -> &str {
        self.peek_line().unwrap_or("")
    }
}

/// Convert CRLF and lone CR to LF.
///
/// This function preserves all other characters verbatim.
fn normalize_newlines(src: &str) -> String {
    src.replace("\r\n", "\n").replace('\r', "\n")
}

/// Compute byte (start, end) for each line, including a trailing '\n' if present.
///
/// Behavior matches `str::lines()` w.r.t. trailing newline: if the input ends
/// with '\n', we do not add an extra empty final line. The key difference is
/// that each line range is contiguous and includes the '\n' character, so there
/// are no gaps between adjacent line ranges.
fn index_lines(buf: &str) -> Vec<Range> {
    let bytes = buf.as_bytes();
    let mut ranges = Vec::new();
    let mut start = 0usize;

    for (idx, &b) in bytes.iter().enumerate() {
        if b == b'\n' {
            // Include the trailing '\n' in the line's range so that ranges are
            // contiguous with no gaps.
            ranges.push(Range {
                start,
                end: idx + 1,
            });
            start = idx + 1;
        }
    }

    // Push the last line if the buffer does not end with '\n'.
    if start < bytes.len() {
        ranges.push(Range {
            start,
            end: bytes.len(),
        });
    }

    ranges
}

/// Returns true if the line is empty or consists only of (Unicode) whitespace.
#[inline]
pub fn is_blank(line: &str) -> bool {
    line.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_navigates_lines_basic() {
        let s = "one\ntwo\nthree";
        let mut ctx = Context::new(s);

        assert_eq!(ctx.line_count(), 3);
        assert!(!ctx.is_eof());

        assert_eq!(ctx.peek_line(), Some("one"));
        assert_eq!(ctx.peek_line_n(1), Some("two"));
        assert_eq!(ctx.peek_line_n(2), Some("three"));
        assert_eq!(ctx.peek_line_n(3), None);

        assert_eq!(ctx.current_line(), "one");
        ctx.advance();
        assert_eq!(ctx.current_line(), "two");
        ctx.advance();
        assert_eq!(ctx.current_line(), "three");
        ctx.advance();

        assert!(ctx.is_eof());
        assert_eq!(ctx.peek_line(), None);
        assert_eq!(ctx.current_line(), "");
    }

    #[test]
    fn crlf_and_cr_are_normalized() {
        let s = "a\r\nb\rc\n";
        let ctx = Context::new(s);

        // No extra trailing empty line when input ends with '\n'
        assert_eq!(ctx.line_count(), 3);
        assert_eq!(ctx.peek_line(), Some("a"));
        assert_eq!(ctx.peek_line_n(1), Some("b"));
        assert_eq!(ctx.peek_line_n(2), Some("c"));
    }

    #[test]
    fn peek_line_n_bounds_and_after_advance() {
        let s = "x\ny";
        let mut ctx = Context::new(s);

        assert_eq!(ctx.peek_line_n(0), Some("x"));
        assert_eq!(ctx.peek_line_n(1), Some("y"));
        assert_eq!(ctx.peek_line_n(2), None);

        ctx.advance();
        assert_eq!(ctx.peek_line_n(0), Some("y"));
        assert_eq!(ctx.peek_line_n(1), None);
    }

    #[test]
    fn unicode_lines_are_preserved() {
        let s = "αβγ\n中文行\nemoji 😀";
        let ctx = Context::new(s);
        assert_eq!(ctx.line_count(), 3);
        assert_eq!(ctx.peek_line(), Some("αβγ"));
        assert_eq!(ctx.peek_line_n(1), Some("中文行"));
        assert_eq!(ctx.peek_line_n(2), Some("emoji 😀"));
    }

    #[test]
    fn is_blank_covers_whitespace() {
        assert!(is_blank(""));
        assert!(is_blank("   "));
        assert!(is_blank("\t  "));
    }
}
