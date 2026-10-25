// NONOS Operating System (AGPL-3.0-or-later)
//! Where the marks go: every match, after the prefix, as the search found it.

use super::{hl, OFF, ON};

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
