// NONOS Operating System (AGPL-3.0-or-later)
//! History expansion, checked on the cases where getting it wrong runs
//! something the reader did not ask for.

use crate::expand::{expand, Entries, ExpandError};

struct Hist(Vec<&'static str>);

impl Entries for Hist {
    fn count(&self) -> usize {
        self.0.len()
    }
    fn get(&self, i: usize) -> &[u8] {
        self.0[i].as_bytes()
    }
}

fn hist() -> Hist {
    Hist(vec!["ls -la", "cargo build", "grep foo src", "cargo test"])
}

fn run(line: &str) -> Option<Result<String, ExpandError>> {
    expand(line.as_bytes(), &hist()).map(|r| r.map(|v| String::from_utf8(v).unwrap()))
}

#[test]
fn a_line_without_a_bang_is_left_alone() {
    assert!(run("ls -la").is_none());
}

#[test]
fn bang_bang_is_the_previous_command() {
    assert_eq!(run("!!"), Some(Ok("cargo test".into())));
}

/// `history` numbers from one, so `!2` must be the second thing printed. An
/// off-by-one here silently runs a neighbour.
#[test]
fn an_index_counts_from_one_as_history_prints_it() {
    assert_eq!(run("!1"), Some(Ok("ls -la".into())));
    assert_eq!(run("!2"), Some(Ok("cargo build".into())));
    assert_eq!(run("!4"), Some(Ok("cargo test".into())));
}

#[test]
fn a_prefix_finds_the_most_recent_match_not_the_oldest() {
    assert_eq!(run("!cargo"), Some(Ok("cargo test".into())));
    assert_eq!(run("!grep"), Some(Ok("grep foo src".into())));
}

#[test]
fn a_designator_expands_inside_a_longer_line() {
    assert_eq!(run("!! && echo done"), Some(Ok("cargo test && echo done".into())));
}

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

// A `!` that was not a designator must survive untouched.

#[test]
fn a_bang_inside_a_word_is_ordinary_text() {
    assert!(run("echo hi!").is_none());
    assert!(run("grep foo!bar file").is_none());
}

#[test]
fn a_bare_bang_is_ordinary_text() {
    assert!(run("echo !").is_none());
}

#[test]
fn a_huge_index_does_not_overflow() {
    assert_eq!(run("!99999999999999999999999999"), Some(Err(ExpandError::NoMatch)));
}
