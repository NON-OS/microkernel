// NONOS Operating System (AGPL-3.0-or-later)
//! The refusals: words that are no command, short names, far names, long tokens.

use super::near;
use crate::suggest::distance;

// The refusals.

#[test]
fn a_word_that_is_not_a_command_gets_no_suggestion() {
    assert_eq!(near("zzzzzzzz"), None);
    assert_eq!(near("kubernetes"), None);
}

/// Two short names are different words, not typos of each other. Offering
/// `cd` for `ls` would be worse than saying nothing.
#[test]
fn short_names_are_not_proposed_for_each_other() {
    assert_eq!(near("xy"), None);
    assert_eq!(near("qq"), None);
}

#[test]
fn a_far_name_is_refused_rather_than_reached_for() {
    // Three edits away from anything in the list.
    assert_eq!(near("grepped"), None);
}

#[test]
fn an_empty_name_suggests_nothing_absurd() {
    assert!(near("").is_none() || near("").is_some_and(|s| s.len() <= 2));
}

/// A very long token is not a mistyped command, and must not cost anything or
/// index past the fixed rows the distance uses.
#[test]
fn an_overlong_token_is_rejected_without_indexing_past_the_rows() {
    let long = "a".repeat(500);
    assert_eq!(near(&long), None);
    assert_eq!(distance(long.as_bytes(), b"ls"), usize::MAX);
}
