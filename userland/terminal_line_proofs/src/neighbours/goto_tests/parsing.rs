// NONOS Operating System (AGPL-3.0-or-later)
//! Parsing what was typed.

use super::{line_at, DOC};
use crate::goto_line::{offset_of_line, parse_line_number};

// Parsing what was typed.

#[test]
fn digits_parse() {
    assert_eq!(parse_line_number(b"1"), Some(1));
    assert_eq!(parse_line_number(b"420"), Some(420));
}

/// Anything that is not entirely digits is refused rather than guessed at. A
/// prompt that read `12abc` as 12 would be inventing intent.
#[test]
fn anything_but_digits_is_refused() {
    assert_eq!(parse_line_number(b"12abc"), None);
    assert_eq!(parse_line_number(b"-4"), None);
    assert_eq!(parse_line_number(b" 4"), None);
    assert_eq!(parse_line_number(b""), None);
}

#[test]
fn an_unrepresentable_number_saturates_rather_than_wrapping() {
    let n = parse_line_number(b"99999999999999999999999999").unwrap();
    assert_eq!(n, usize::MAX);
    // And it still lands somewhere real.
    assert_eq!(line_at(DOC, offset_of_line(DOC, n)), "four");
}
