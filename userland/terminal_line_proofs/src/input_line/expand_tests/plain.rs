// NONOS Operating System (AGPL-3.0-or-later)
//! A `!` that was not a designator survives untouched.

use super::run;
use crate::expand::ExpandError;

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
