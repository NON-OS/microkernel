// NONOS Operating System (AGPL-3.0-or-later)
//! The filters that already existed, covered so the whole module is exercised.

use super::{lines, show};
use crate::text::{grep, sort};

#[test]
fn sort_orders_lines_and_honours_its_flags() {
    assert_eq!(show(sort(&[], lines(&["b", "a", "c"]))), vec!["a", "b", "c"]);
    assert_eq!(show(sort(&[b"-r"], lines(&["a", "b"]))), vec!["b", "a"]);
    assert_eq!(show(sort(&[b"-u"], lines(&["b", "a", "b"]))), vec!["a", "b"]);
}

/// Numeric order is not string order: `10` sorts after `9`, which is the whole
/// reason `-n` exists.
#[test]
fn numeric_sort_orders_by_value_not_by_digit() {
    assert_eq!(show(sort(&[b"-n"], lines(&["10", "9", "100"]))), vec!["9", "10", "100"]);
}

#[test]
fn grep_keeps_matching_lines() {
    assert_eq!(show(grep(&[b"a"], lines(&["cat", "dog", "bat"]))), vec!["cat", "bat"]);
}

#[test]
fn grep_inverts_and_ignores_case_when_told() {
    assert_eq!(show(grep(&[b"-v", b"a"], lines(&["cat", "dog"]))), vec!["dog"]);
    assert_eq!(show(grep(&[b"-i", b"CAT"], lines(&["cat", "dog"]))), vec!["cat"]);
}
