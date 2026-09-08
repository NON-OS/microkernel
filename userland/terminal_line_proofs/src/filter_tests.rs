// NONOS Operating System (AGPL-3.0-or-later)
//! The pipeline text filters.

use crate::text::{rev, tac, uniq};

fn lines(v: &[&str]) -> Vec<Vec<u8>> {
    v.iter().map(|s| s.as_bytes().to_vec()).collect()
}

fn show(v: Vec<Vec<u8>>) -> Vec<String> {
    v.into_iter().map(|l| String::from_utf8(l).unwrap()).collect()
}

#[test]
fn uniq_collapses_only_neighbours() {
    // Not a set: `a b a` keeps both a's, which is what makes `sort | uniq`
    // the idiom rather than `uniq` alone.
    assert_eq!(show(uniq(&[], lines(&["a", "a", "b", "a"]))), vec!["a", "b", "a"]);
}

#[test]
fn uniq_counts_each_run_when_asked() {
    let out = show(uniq(&[b"-c"], lines(&["a", "a", "a", "b"])));
    assert_eq!(out.len(), 2);
    assert!(out[0].ends_with(" a"), "{:?}", out[0]);
    assert!(out[0].trim_start().starts_with('3'), "{:?}", out[0]);
    assert!(out[1].trim_start().starts_with('1'), "{:?}", out[1]);
}

/// The counts form their own column, so a reader scans down them.
#[test]
fn counted_lines_align_in_a_column() {
    let out = show(uniq(&[b"-c"], lines(&["x", "y"])));
    let at = |s: &String| s.find(|c: char| !c.is_whitespace()).unwrap();
    assert_eq!(at(&out[0]), at(&out[1]));
}

#[test]
fn uniq_of_nothing_is_nothing() {
    assert!(uniq(&[], lines(&[])).is_empty());
    assert!(uniq(&[b"-c"], lines(&[])).is_empty());
}

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

// The filters that already existed, covered here so the whole module is
// exercised rather than half of it.

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
