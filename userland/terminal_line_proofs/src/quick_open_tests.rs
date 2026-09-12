// NONOS Operating System (AGPL-3.0-or-later)
//! Quick open: finding a file from a few letters. The ranking is the feature.
//! A matcher that finds the right file second is one nobody trusts.

use crate::quick_open::{best, score, subsequence};

const TREE: &[&[u8]] = &[
    b"/README",
    b"/src/main.rs",
    b"/src/editor/mod.rs",
    b"/src/editor/paint.rs",
    b"/docs/main/notes.md",
];

fn find(q: &str) -> Option<&'static str> {
    best(TREE, q.as_bytes()).map(|i| core::str::from_utf8(TREE[i]).unwrap())
}

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

/// The ranking rule that matters: a hit in the file's own name beats one that
/// only lines up across the directories above it. Typing `main` means
/// `src/main.rs`, not a notes file living under a folder called main.
#[test]
fn a_name_match_beats_a_path_match() {
    assert_eq!(find("main"), Some("/src/main.rs"));
}

#[test]
fn the_shorter_path_wins_when_both_match_the_name() {
    // Both have `mod`/`paint` in the name; only one has `paint`.
    assert_eq!(find("paint"), Some("/src/editor/paint.rs"));
}

#[test]
fn a_few_letters_find_a_nested_file() {
    assert_eq!(find("edmod"), Some("/src/editor/mod.rs"));
}

#[test]
fn nothing_matching_returns_nothing() {
    assert_eq!(find("zzzz"), None);
}

#[test]
fn a_query_longer_than_every_path_matches_nothing() {
    assert_eq!(find(&"x".repeat(300)), None);
}

/// Two searches for the same thing must give the same answer, or the feature
/// is unusable by muscle memory.
#[test]
fn the_answer_is_stable() {
    for _ in 0..10 {
        assert_eq!(find("main"), Some("/src/main.rs"));
    }
}

#[test]
fn a_path_match_still_scores_when_no_name_matches() {
    // `docs` appears only in the directory part.
    assert!(score(b"/docs/main/notes.md", b"docs").is_some());
    // And it ranks worse than any name match for the same query.
    let path_only = score(b"/docs/main/notes.md", b"main").unwrap();
    let name_hit = score(b"/src/main.rs", b"main").unwrap();
    assert!(name_hit < path_only);
}

#[test]
fn scoring_a_non_match_is_none() {
    assert_eq!(score(b"/src/main.rs", b"qqq"), None);
}
