// NONOS Operating System (AGPL-3.0-or-later)
//! The pipe filters take the flags `help` gives them. In a pipe `head -n 3`
//! printed ten lines, `wc -w` counted lines and `grep -c` took `-c` for the
//! pattern; `cut -d , -f 2` with spaces read neither value.

use crate::count::{head, line_count, tail, wc, wc_row};
use crate::input::{lines_of, not_a_filter, without, FILTERS};
use crate::text::{cut, grep};

fn lines(v: &[&str]) -> Vec<Vec<u8>> {
    v.iter().map(|s| s.as_bytes().to_vec()).collect()
}

fn show(v: Vec<Vec<u8>>) -> Vec<String> {
    v.into_iter().map(|l| String::from_utf8(l).unwrap()).collect()
}

fn ten() -> Vec<Vec<u8>> {
    (1..=12).map(|i| format!("l{i}").into_bytes()).collect()
}

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

#[test]
fn grep_in_a_pipe_counts_and_numbers() {
    let input = lines(&["apple", "berry", "apricot"]);
    assert_eq!(show(grep(&[b"-c", b"ap"], input.clone())), vec!["2"]);
    assert_eq!(show(grep(&[b"-n", b"ap"], input.clone())), vec!["1:apple", "3:apricot"]);
    assert_eq!(show(grep(&[b"-vn", b"ap"], input.clone())), vec!["2:berry"]);
    assert_eq!(show(grep(&[b"-r", b"ap"], input.clone())), vec!["grep: unknown flag -r"]);
    assert_eq!(show(grep(&[], input)), vec!["grep: missing pattern"]);
}

#[test]
fn cut_reads_its_values_joined_or_apart() {
    let input = lines(&["a,b,c", "d,e"]);
    assert_eq!(show(cut(&[b"-d", b",", b"-f", b"2"], input.clone())), vec!["b", "e"]);
    assert_eq!(show(cut(&[b"-d,", b"-f3"], input)), vec!["c", ""]);
}

/// `sort f` sorted the empty "line" after a file's last newline to the top.
#[test]
fn a_file_s_last_newline_ends_a_line_rather_than_adding_one() {
    assert_eq!(show(lines_of(b"b\na\n")), vec!["b", "a"]);
    assert_eq!(show(lines_of(b"b\na")), vec!["b", "a"]);
    assert_eq!(show(lines_of(b"\n")), vec![""]);
    assert!(lines_of(b"").is_empty());
}

#[test]
fn files_named_after_a_filter_are_taken_off_its_arguments() {
    let args: Vec<&[u8]> = vec![b"cut", b"-d", b",", b"f.csv"];
    let files: Vec<&[u8]> = vec![args[3]];
    assert_eq!(without(&args, &files), vec![&b"cut"[..], b"-d", b","]);
}

/// After a `|` a command that is not a filter used to run as if nothing had
/// been piped, and the piped lines were lost without a word.
#[test]
fn a_command_that_cannot_read_a_pipe_says_so_and_names_those_that_can() {
    let msg = String::from_utf8(not_a_filter(b"ls")).unwrap();
    assert!(msg.starts_with("pipe: ls does not read from a pipe"));
    for f in FILTERS {
        assert!(msg.contains(std::str::from_utf8(f).unwrap()));
    }
}
