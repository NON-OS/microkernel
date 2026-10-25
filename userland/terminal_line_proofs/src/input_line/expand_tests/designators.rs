// NONOS Operating System (AGPL-3.0-or-later)
//! The designators that expand: `!!`, an index, a prefix, inside a longer line.

use super::run;

#[test]
fn a_line_without_a_bang_is_left_alone() {
    assert!(run("ls -la").is_none());
}

#[test]
fn bang_bang_is_the_previous_command() {
    assert_eq!(run("!!"), Some(Ok("cargo test".into())));
}

/// `history` numbers from one, so `!2` must be the second thing printed. An
/// off-by-one here silently runs a neighbour.
#[test]
fn an_index_counts_from_one_as_history_prints_it() {
    assert_eq!(run("!1"), Some(Ok("ls -la".into())));
    assert_eq!(run("!2"), Some(Ok("cargo build".into())));
    assert_eq!(run("!4"), Some(Ok("cargo test".into())));
}

#[test]
fn a_prefix_finds_the_most_recent_match_not_the_oldest() {
    assert_eq!(run("!cargo"), Some(Ok("cargo test".into())));
    assert_eq!(run("!grep"), Some(Ok("grep foo src".into())));
}

#[test]
fn a_designator_expands_inside_a_longer_line() {
    assert_eq!(run("!! && echo done"), Some(Ok("cargo test && echo done".into())));
}
