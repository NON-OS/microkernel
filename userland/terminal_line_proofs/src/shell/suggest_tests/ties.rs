// NONOS Operating System (AGPL-3.0-or-later)
//! Two names, when two are equally near.

use super::{near, NAMES};
use crate::suggest::nearest_two;

// Two names, when two are equally near.

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
