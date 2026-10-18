// NONOS Operating System (AGPL-3.0-or-later)
//! The cases where marking would be wrong: no match, odd patterns and prefixes.

use super::{hl, OFF, ON};

#[test]
fn no_match_returns_nothing_so_the_plain_row_is_used() {
    assert_eq!(hl("dog", 0, "cat", false), None);
}

#[test]
fn an_empty_pattern_marks_nothing_rather_than_looping() {
    assert_eq!(hl("anything", 0, "", false), None);
}

#[test]
fn a_pattern_longer_than_the_line_is_no_match() {
    assert_eq!(hl("ab", 0, "abcdef", false), None);
}

/// A prefix longer than the row is a caller error, and must return nothing
/// rather than slicing past the end.
#[test]
fn an_out_of_range_prefix_does_not_panic() {
    assert_eq!(hl("short", 99, "s", false), None);
}

/// Whatever else it does, the visible text must survive: stripping the escapes
/// has to give back the original row.
#[test]
fn stripping_the_marks_returns_the_original_line() {
    for (row, prefix, needle) in
        [("a cat here", 0usize, "cat"), ("f.txt:2:cat cat", 8, "cat"), ("cat", 0, "cat")]
    {
        let out = hl(row, prefix, needle, false).unwrap();
        assert_eq!(out.replace(ON, "").replace(OFF, ""), row);
    }
}
