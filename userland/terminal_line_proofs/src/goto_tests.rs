// NONOS Operating System (AGPL-3.0-or-later)
//! Go to line. An editor that lands one line from where it was told is worse
//! than one that cannot jump, so the boundaries are what matter here.

use crate::goto_line::{offset_of_line, parse_line_number};

const DOC: &[u8] = b"one\ntwo\nthree\nfour";

fn line_at(buf: &[u8], off: usize) -> String {
    let end = buf[off..].iter().position(|&b| b == b'\n').map(|p| off + p).unwrap_or(buf.len());
    String::from_utf8(buf[off..end].to_vec()).unwrap()
}

#[test]
fn line_one_is_the_start_of_the_document() {
    assert_eq!(offset_of_line(DOC, 1), 0);
    assert_eq!(line_at(DOC, offset_of_line(DOC, 1)), "one");
}

#[test]
fn each_line_lands_on_its_own_first_byte() {
    for (n, expect) in [(1, "one"), (2, "two"), (3, "three"), (4, "four")] {
        assert_eq!(line_at(DOC, offset_of_line(DOC, n)), expect, "line {n}");
    }
}

/// One-based, the way every compiler and every colleague counts. Line 0 is not
/// a thing anyone means, so it goes to the top rather than underflowing.
#[test]
fn line_zero_is_treated_as_the_first_line() {
    assert_eq!(offset_of_line(DOC, 0), 0);
}

/// Past the end lands on the last line, not after the last byte, so the caret
/// is somewhere readable.
#[test]
fn past_the_end_lands_on_the_final_line() {
    let off = offset_of_line(DOC, 900);
    assert_eq!(line_at(DOC, off), "four");
    assert!(off < DOC.len());
}

#[test]
fn a_document_with_no_newlines_has_exactly_one_line() {
    assert_eq!(offset_of_line(b"single", 1), 0);
    assert_eq!(offset_of_line(b"single", 5), 0);
}

#[test]
fn an_empty_document_does_not_index_past_itself() {
    assert_eq!(offset_of_line(b"", 1), 0);
    assert_eq!(offset_of_line(b"", 50), 0);
}

/// A trailing newline means there is an empty last line, and asking for it
/// should land on it rather than on the line before.
#[test]
fn a_trailing_newline_makes_a_real_final_line() {
    let doc = b"a\nb\n";
    assert_eq!(offset_of_line(doc, 3), 4);
    assert_eq!(line_at(doc, offset_of_line(doc, 3)), "");
}

#[test]
fn consecutive_newlines_are_separate_lines() {
    let doc = b"a\n\n\nb";
    assert_eq!(line_at(doc, offset_of_line(doc, 2)), "");
    assert_eq!(line_at(doc, offset_of_line(doc, 4)), "b");
}

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
