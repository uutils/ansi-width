[![Crates.io](https://img.shields.io/crates/v/ansi-width.svg)](https://crates.io/crates/ansi-width)
[![Discord](https://img.shields.io/badge/discord-join-7289DA.svg?logo=discord&longCache=true&style=flat)](https://discord.gg/wQVJbvJ)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/uutils/ansi-width/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/uutils/ansi-width/status.svg)](https://deps.rs/repo/github/uutils/ansi-width)

[![CodeCov](https://codecov.io/gh/uutils/ansi-width/branch/main/graph/badge.svg)](https://codecov.io/gh/uutils/ansi-width)

# ANSI width

Measure the width of a string when printed to the terminal

For ASCII, this is identical to the length of the string in bytes. However,
there are 2 special cases:

- Many unicode characters (CJK, emoji, etc.) span multiple columns.
- ANSI escape codes should be ignored.

The first case is handled by the `unicode-width` crate. This function extends
that crate by ignoring ANSI escape codes.

## Limitations

- We cannot know the width of a `TAB` character in the terminal emulator.
- Backspace is also treated as zero width.

## A Primer on ANSI escape codes (and how this crate works)

ANSI codes are created using special character sequences in a string. These
sequences start with the ESC character: `'\x1b'`, followed by some other
character to determine the type of the escape code. That second character
determines how long the sequence continues:

- `ESC [`: until a character in the range `'\x40'..='\x7E'` is found.
- `ESC ]`: until an `ST` is found.

An `ST` is a String Terminator and is given by the sequence `ESC \` (or in Rust
syntax `'\x1b\x5c'`).

This is the subset of sequences that this library supports, since these are used
by most applications that need this functionality. If you have a use case for
other codes, please open an issue on the
[GitHub repository](https://github.com/uutils/ansi-width).

`ansi-width` does not parse the actual ANSI codes to improve performance, it can
only skip the ANSI codes.

## Examples

```rust
use ansi_width::ansi_width;

// ASCII string
assert_eq!(ansi_width("123456"), 6);

// Accents
assert_eq!(ansi_width("café"), 4);

// Emoji (2 crab emoji)
assert_eq!(ansi_width("🦀🦀"), 4);

// CJK characters (“Nǐ hǎo” or “Hello” in Chinese)
assert_eq!(ansi_width("你好"), 4);

// ANSI colors
assert_eq!(ansi_width("\u{1b}[31mRed\u{1b}[0m"), 3);

// ANSI hyperlink
assert_eq!(
    ansi_width("\x1b]8;;http://example.com\x1b\\This is a link\x1b]8;;\x1b\\"),
    14
);
```

## Alternatives

- [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len): Returns only the length in bytes and therefore only works for
  ASCII characters.
- [`unicode-width`](https://crates.io/crates/unicode-width): Does not take ANSI
  characters into account by design (see
  [this issue](https://github.com/unicode-rs/unicode-width/issues/24)). This
  might be what you want if you don't care about ANSI codes. `unicode-width` is
  used internally by this crate as well.
- [`textwrap::core::display_width`](https://docs.rs/textwrap/latest/textwrap/core/fn.display_width.html):
  Very similar functionality to this crate and it also supports hyperlinks since version 0.16.1. The 
  advantage of this crate is that it does not require pulling in the rest of `textwrap`'s functionality
  (even though that functionality is excellent if you need it).
- [`console::measure_text_width`](https://docs.rs/console/latest/console/fn.measure_text_width.html):
  Similar to `textwrap` and very well-tested. However, it constructs a new
  string internally without ANSI codes first and then measures the width of
  that. The parsing is more robust than this crate though.

## References

The information above is based on:

- <https://en.wikipedia.org/wiki/ANSI_escape_code>
- <https://www.ecma-international.org/wp-content/uploads/ECMA-48_5th_edition_june_1991.pdf>
