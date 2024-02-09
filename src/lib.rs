#![doc = include_str!("../README.md")]

/// Character that starts escape codes
const ESC: char = '\x1b';

/// Calculate the width of a string.
///
/// See the [crate documentation](crate) for more information.
pub fn ansi_width(s: &str) -> usize {
    let mut width = 0;
    let mut chars = s.chars();

    // This lint is a false positive, because we use the iterator later, leading to
    // ownership issues if we follow the lint.
    #[allow(clippy::while_let_on_iterator)]
    while let Some(c) = chars.next() {
        // ESC starts escape sequences, so we need to take characters until the
        // end of the escape sequence.
        if c == ESC {
            let Some(c) = chars.next() else {
                break;
            };
            match c {
                // String terminator character: ends other sequences
                // We probably won't encounter this but it's here for completeness.
                // Or for if we get passed invalid codes.
                '\\' => {
                    // ignore
                }
                // Control Sequence Introducer: continue until `\x40-\x7C`
                '[' => while !matches!(chars.next(), Some('\x40'..='\x7C') | None) {},
                // Operating System Command: continue until ST
                ']' => {
                    let mut last = c;
                    while let Some(new) = chars.next() {
                        if new == '\x07' || (new == '\\' && last == ESC) {
                            break;
                        }
                        last = new;
                    }
                }
                // We don't know what character it is, best bet is to fall back to unicode width
                // The ESC is assumed to have 0 width in this case.
                _ => {
                    width += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
                }
            }
        } else {
            // If it's a normal character outside an escape sequence, use the
            // unicode width.
            width += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        }
    }
    width
}

#[cfg(test)]
mod tests {
    use super::ansi_width;

    #[test]
    fn ascii() {
        assert_eq!(ansi_width(""), 0);
        assert_eq!(ansi_width("hello"), 5);
        assert_eq!(ansi_width("hello world"), 11);
        assert_eq!(ansi_width("WOW!"), 4);
    }

    #[test]
    fn c0_characters() {
        // Bell
        assert_eq!(ansi_width("\x07"), 0);

        // Backspace
        assert_eq!(ansi_width("\x08"), 0);

        // Tab
        assert_eq!(ansi_width("\t"), 0);
    }

    #[test]
    fn some_escape_codes() {
        // Simple
        assert_eq!(ansi_width("\u{1b}[34mHello\u{1b}[0m"), 5);
        // Red
        assert_eq!(ansi_width("\u{1b}[31mRed\u{1b}[0m"), 3);
    }

    #[test]
    fn hyperlink() {
        assert_eq!(
            ansi_width("\x1b]8;;http://example.com\x1b\\This is a link\x1b]8;;\x1b\\"),
            14
        )
    }

    #[test]
    fn nonstandard_hyperlink() {
        // This hyperlink has a BEL character in the middle instead of `\x1b\\`
        assert_eq!(
            ansi_width("\x1b]8;;file://coreutils.md\x07coreutils.md\x1b]8;;\x07"),
            12
        )
    }
}
