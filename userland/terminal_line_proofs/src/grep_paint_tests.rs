// NONOS Operating System (AGPL-3.0-or-later)
//! Marking grep's matches inside the line, and the cases where marking would
//! be wrong.

use crate::grep_paint::highlight;

const ON: &str = "\x1b[1;33m";
const OFF: &str = "\x1b[0m";

fn hl(row: &str, prefix: usize, needle: &str, fold: bool) -> Option<String> {
    highlight(row.as_bytes(), prefix, needle.as_bytes(), fold)
        .map(|v| String::from_utf8(v).unwrap())
}

#[test]
fn the_match_is_wrapped_and_the_rest_is_untouched() {
    assert_eq!(hl("a cat here", 0, "cat", false), Some(format!("a {ON}cat{OFF} here")));
}

#[test]
fn every_occurrence_is_marked() {
    let out = hl("cat cat", 0, "cat", false).unwrap();
    assert_eq!(out.matches(ON).count(), 2);
    assert_eq!(out.matches(OFF).count(), 2);
}

/// The file name and line number are not part of the line's text. A pattern
/// that happens to appear in a path must not be marked as though it were a hit.
#[test]
fn the_prefix_is_never_marked() {
    let row = "cat.txt:1:the dog";
    let out = hl(row, 10, "cat", false);
    assert_eq!(out, None, "matched inside the file name");
}

#[test]
fn a_match_after_the_prefix_is_still_found() {
    let row = "notes.txt:1:the cat";
    let out = hl(row, 12, "cat", false).unwrap();
    assert!(out.starts_with("notes.txt:1:"));
    assert!(out.ends_with(&format!("{ON}cat{OFF}")));
}

#[test]
fn case_folding_marks_what_the_search_actually_found() {
    assert_eq!(hl("A Cat", 0, "cat", true), Some(format!("A {ON}Cat{OFF}")));
    // Without folding the same line has nothing to mark.
    assert_eq!(hl("A Cat", 0, "cat", false), None);
}

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

/// Overlapping candidates must not double-mark: `aa` in `aaa` is one match
/// then a leftover, never two marks over the same byte.
#[test]
fn matches_do_not_overlap() {
    let out = hl("aaa", 0, "aa", false).unwrap();
    assert_eq!(out.matches(ON).count(), 1);
    assert_eq!(out, format!("{ON}aa{OFF}a"));
}

#[test]
fn the_whole_line_matching_is_handled() {
    assert_eq!(hl("cat", 0, "cat", false), Some(format!("{ON}cat{OFF}")));
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
