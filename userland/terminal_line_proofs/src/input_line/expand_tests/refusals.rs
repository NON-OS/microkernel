// NONOS Operating System (AGPL-3.0-or-later)
//! The cases where expansion must refuse rather than guess.

use super::{run, Hist};
use crate::expand::{expand, ExpandError};

// The cases where expansion must refuse rather than guess.

#[test]
fn an_index_past_the_end_refuses() {
    assert_eq!(run("!99"), Some(Err(ExpandError::NoMatch)));
}

#[test]
fn a_prefix_that_matches_nothing_refuses() {
    assert_eq!(run("!zzz"), Some(Err(ExpandError::NoMatch)));
}

#[test]
fn index_zero_refuses_rather_than_wrapping_to_the_end() {
    assert_eq!(run("!0"), Some(Err(ExpandError::NoMatch)));
}

#[test]
fn bang_bang_with_no_history_refuses() {
    let empty = Hist(vec![]);
    assert_eq!(
        expand(b"!!", &empty).map(|r| r.map(|v| String::from_utf8(v).unwrap())),
        Some(Err(ExpandError::NoMatch))
    );
}
