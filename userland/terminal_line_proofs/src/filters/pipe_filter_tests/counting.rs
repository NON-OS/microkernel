// NONOS Operating System (AGPL-3.0-or-later)
//! `head`, `tail` and `wc` in a pipe take their counts as `help` gives them.

use super::{lines, show, ten};
use crate::count::{head, line_count, tail, wc, wc_row};

#[test]
fn head_and_tail_take_their_count_in_every_spelling() {
    for args in [&[&b"-n"[..], b"3"][..], &[b"-n3"], &[b"-3"], &[b"3"]] {
        assert_eq!(show(head(args, ten())), vec!["l1", "l2", "l3"], "{args:?}");
        assert_eq!(show(tail(args, ten())), vec!["l10", "l11", "l12"], "{args:?}");
    }
    assert_eq!(head(&[], ten()).len(), 10);
}

#[test]
fn a_bad_count_is_said_rather_than_ignored() {
    assert_eq!(show(head(&[b"-n", b"many"], ten())), vec!["head: -n takes a count"]);
    assert!(line_count(b"tail", &[b"-x"]).is_err());
    assert!(line_count(b"head", &[b"file.txt"]).is_err());
}

#[test]
fn wc_in_a_pipe_counts_what_it_is_asked_for() {
    let input = lines(&["one two", "three"]);
    assert_eq!(show(wc(&[], input.clone())), vec!["lines 2  words 3  bytes 14"]);
    assert_eq!(show(wc(&[b"-w"], input.clone())), vec!["words 3"]);
    assert_eq!(show(wc(&[b"-l", b"-c"], input)), vec!["lines 2  bytes 14"]);
    assert_eq!(wc_row(1, 2, 3, [false, true, false]), b"words 2".to_vec());
}
