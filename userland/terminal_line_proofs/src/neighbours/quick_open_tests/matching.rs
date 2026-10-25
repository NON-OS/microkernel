// NONOS Operating System (AGPL-3.0-or-later)
//! Which paths match: letters in order, any case, and nothing for a non-match.

use super::find;
use crate::quick_open::{score, subsequence};

#[test]
fn letters_in_order_match_though_not_together() {
    assert!(subsequence(b"main.rs", b"mn"));
    assert!(subsequence(b"/src/editor/mod.rs", b"edmod"));
}

#[test]
fn letters_out_of_order_do_not_match() {
    assert!(!subsequence(b"main.rs", b"nm"));
}

#[test]
fn matching_ignores_case() {
    assert!(subsequence(b"README", b"rdm"));
    assert!(subsequence(b"main.rs", b"MAIN"));
}

#[test]
fn an_empty_query_matches_everything() {
    assert!(subsequence(b"anything", b""));
    assert!(find("").is_some());
}

#[test]
fn nothing_matching_returns_nothing() {
    assert_eq!(find("zzzz"), None);
}

#[test]
fn a_query_longer_than_every_path_matches_nothing() {
    assert_eq!(find(&"x".repeat(300)), None);
}

#[test]
fn scoring_a_non_match_is_none() {
    assert_eq!(score(b"/src/main.rs", b"qqq"), None);
}
