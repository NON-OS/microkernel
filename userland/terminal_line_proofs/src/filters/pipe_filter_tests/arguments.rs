// NONOS Operating System (AGPL-3.0-or-later)
//! `grep` and `cut` flags in a pipe, the files a filter is named, and a stage that is no filter.

use super::{lines, show};
use crate::input::{lines_of, not_a_filter, without, FILTERS};
use crate::text::{cut, grep};

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
