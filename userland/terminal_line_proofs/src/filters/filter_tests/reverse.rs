// NONOS Operating System (AGPL-3.0-or-later)
//! `tac` and `rev`: the order of lines, and the bytes within one.

use super::{lines, show};
use crate::text::{rev, tac};

#[test]
fn tac_reverses_the_order_of_lines() {
    assert_eq!(show(tac(lines(&["one", "two", "three"]))), vec!["three", "two", "one"]);
}

#[test]
fn rev_reverses_within_a_line_and_not_between() {
    assert_eq!(show(rev(lines(&["abc", "de"]))), vec!["cba", "ed"]);
}

#[test]
fn tac_and_rev_are_their_own_inverses() {
    let input = lines(&["alpha", "beta", "gamma"]);
    assert_eq!(show(tac(tac(input.clone()))), show(input.clone()));
    assert_eq!(show(rev(rev(input.clone()))), show(input));
}
