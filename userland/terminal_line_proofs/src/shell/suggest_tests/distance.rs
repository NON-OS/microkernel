// NONOS Operating System (AGPL-3.0-or-later)
//! The edit distance: zero to itself, and a swapped pair counts as one edit.

use super::NAMES;
use crate::suggest::distance;

#[test]
fn distance_of_a_name_to_itself_is_zero() {
    for n in NAMES {
        assert_eq!(distance(n, n), 0);
    }
}

/// A swapped pair is one edit, not two. Without this `hlep` ties with `grep`
/// and the shell proposes a command with nothing to do with what was typed.
#[test]
fn a_transposition_counts_as_one_edit() {
    assert_eq!(distance(b"hlep", b"help"), 1);
    assert_eq!(distance(b"gerp", b"grep"), 1);
    assert_eq!(distance(b"tokie", b"tokei"), 1);
}

#[test]
fn a_transposition_beats_an_unrelated_name_at_the_same_letters() {
    // Two substitutions away from grep, one swap away from help.
    assert!(distance(b"hlep", b"help") < distance(b"hlep", b"grep"));
}
