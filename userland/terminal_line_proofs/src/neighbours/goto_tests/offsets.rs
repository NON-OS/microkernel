// NONOS Operating System (AGPL-3.0-or-later)
//! Where each line number lands in the document, at the boundaries.

use super::{line_at, DOC};
use crate::goto_line::offset_of_line;

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
