// NONOS Operating System (AGPL-3.0-or-later)
//! Which match comes first: the name over the path, the shorter path, stably.

use super::find;
use crate::quick_open::score;

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
