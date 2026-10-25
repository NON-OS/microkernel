// NONOS Operating System (AGPL-3.0-or-later)
//! `uniq`: neighbours collapsed, runs counted, counts aligned.

use super::{lines, show};
use crate::text::uniq;

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
