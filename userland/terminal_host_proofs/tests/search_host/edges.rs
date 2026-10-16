// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Empty needles, misses, overlong needles and empty history.

use crate::support::history;

/// An empty search shows the newest line, which is what pressing up once
/// would give, rather than nothing.
#[test]
fn an_empty_search_shows_the_newest_line() {
    let h = history(&["one", "two"]);
    let (_, line) = h.search_back(b"", 2).expect("an empty needle matches");
    assert_eq!(line, b"two");
}

#[test]
fn a_needle_that_matches_nothing_finds_nothing() {
    let h = history(&["ls", "pwd"]);
    assert!(h.search_back(b"zzz", 2).is_none());
}

/// A needle longer than any line cannot match, and must not read past the
/// end of one looking.
#[test]
fn a_needle_longer_than_the_line_is_refused() {
    let h = history(&["ls"]);
    assert!(h.search_back(b"lslslslsls", 1).is_none());
}

#[test]
fn searching_an_empty_history_finds_nothing() {
    let h = history(&[]);
    assert!(h.search_back(b"x", 0).is_none());
}
