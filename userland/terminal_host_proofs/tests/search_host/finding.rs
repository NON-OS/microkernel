// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Finding a match and stepping back to older ones.

use crate::support::history;

/// The point of a search rather than a prefix recall: what a reader
/// remembers is usually a word from the middle of a command.
#[test]
fn a_word_from_the_middle_finds_the_line() {
    let h = history(&["ls /tmp", "curl https://example.com", "echo hi"]);
    let (_, line) = h.search_back(b"example", 3).expect("the middle word must match");
    assert_eq!(line, b"curl https://example.com");
}

/// Newest first, because the command a reader wants again is usually the one
/// they ran most recently.
#[test]
fn the_newest_match_comes_first() {
    let h = history(&["git status", "git log", "ls"]);
    let (at, line) = h.search_back(b"git", 3).expect("a match must be found");
    assert_eq!(line, b"git log");
    assert_eq!(at, 1);
}

/// Pressing the key again has to move, which is what stepping past the match
/// on screen means.
#[test]
fn stepping_back_finds_the_older_match() {
    let h = history(&["git status", "git log", "ls"]);
    let (at, _) = h.search_back(b"git", 3).expect("first match");
    let (_, older) = h.search_back(b"git", at).expect("an older match must be found");
    assert_eq!(older, b"git status");
}
