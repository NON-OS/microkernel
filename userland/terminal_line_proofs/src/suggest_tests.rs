// NONOS Operating System (AGPL-3.0-or-later)
//! "did you mean". A suggester that proposes nonsense is worse than silence,
//! so the refusals matter more here than the hits.

use crate::suggest::{distance, nearest};

const NAMES: &[&[u8]] = &[
    b"ls",
    b"cat",
    b"cd",
    b"grep",
    b"help",
    b"history",
    b"tree",
    b"type",
    b"which",
    b"capsules",
    b"clear",
    b"echo",
    b"exit",
    b"mkdir",
    b"touch",
    b"tokei",
    b"rg",
];

fn near(typed: &str) -> Option<String> {
    nearest(typed.as_bytes(), NAMES.iter().copied()).map(|c| String::from_utf8(c.to_vec()).unwrap())
}

#[test]
fn distance_of_a_name_to_itself_is_zero() {
    for n in NAMES {
        assert_eq!(distance(n, n), 0);
    }
}

#[test]
fn a_single_slip_is_caught() {
    assert_eq!(near("gerp"), Some("grep".into()));
    assert_eq!(near("hlep"), Some("help".into()));
    assert_eq!(near("mkdri"), Some("mkdir".into()));
}

#[test]
fn a_missing_letter_is_caught() {
    assert_eq!(near("cler"), Some("clear".into()));
    assert_eq!(near("tre"), Some("tree".into()));
}

#[test]
fn an_extra_letter_is_caught() {
    assert_eq!(near("treee"), Some("tree".into()));
    assert_eq!(near("catt"), Some("cat".into()));
}

/// The point of the feature: names carried over from another system.
#[test]
fn a_neighbouring_real_command_is_offered() {
    assert_eq!(near("historyy"), Some("history".into()));
}

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

#[test]
fn suggestions_are_stable_rather_than_order_dependent() {
    let first = near("gerp");
    for _ in 0..10 {
        assert_eq!(near("gerp"), first);
    }
}

/// Installed tools are candidates too: `tokie` should find `tokei`.
#[test]
fn installed_tools_are_offered_like_any_other_name() {
    assert_eq!(near("tokie"), Some("tokei".into()));
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

// Two names, when two are equally near.

use crate::suggest::nearest_two;

fn near2(typed: &str) -> (Option<String>, Option<String>) {
    let (a, b) = nearest_two(typed.as_bytes(), NAMES.iter().copied());
    let s = |o: Option<&[u8]>| o.map(|c| String::from_utf8(c.to_vec()).unwrap());
    (s(a), s(b))
}

/// The case from a real session: `io` is one edit from both `in` and `ip`, and
/// naming only the first in the table is a coin flip dressed as advice.
#[test]
fn an_exact_tie_offers_both() {
    const PAIR: &[&[u8]] = &[b"in", b"ip"];
    let (a, b) = nearest_two(b"io", PAIR.iter().copied());
    assert_eq!(a, Some(&b"in"[..]));
    assert_eq!(b, Some(&b"ip"[..]));
}

/// A worse match is not a second answer, it is a longer list.
#[test]
fn a_runner_up_that_is_further_away_is_not_offered() {
    let (a, b) = near2("gerp");
    assert_eq!(a, Some("grep".into()));
    assert_eq!(b, None);
}

#[test]
fn a_lone_match_has_no_alternative() {
    let (a, b) = near2("hlep");
    assert_eq!(a, Some("help".into()));
    assert_eq!(b, None);
}

#[test]
fn no_match_offers_neither() {
    assert_eq!(near2("zzzzzzzz"), (None, None));
}

/// Whatever else changes, the single-answer helper must agree with the pair.
#[test]
fn the_single_answer_is_the_first_of_the_pair() {
    for q in ["gerp", "hlep", "tokie", "zzzz", "io"] {
        assert_eq!(near(q), near2(q).0);
    }
}
