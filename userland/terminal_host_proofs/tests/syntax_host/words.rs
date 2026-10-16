// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Command, flag, path and plain words, and commands after operators.

use crate::support::{parts_of, THEME};
use crate::syntax::Part;

/// The command is the one word that decides what the rest of the line means,
/// so it is the one that carries the accent.
#[test]
fn the_first_word_is_the_command() {
    let p = parts_of("ls -la /etc");
    assert_eq!(p[0], Part::Command);
    assert_eq!(p[1], Part::Command);
    assert_eq!(p[2], Part::Plain, "the space is not part of it");
    assert_eq!(p[0].colour(&THEME), THEME.accent);
}

#[test]
fn a_dashed_word_is_a_flag() {
    let p = parts_of("ls -la");
    assert_eq!(p[3], Part::Flag);
    assert_eq!(p[5], Part::Flag);
}

/// A path is coloured from its first character, not from the slash part way
/// along it.
#[test]
fn a_word_with_a_separator_is_a_path_from_its_start() {
    let p = parts_of("cat etc/hosts");
    assert_eq!(p[4], Part::Path, "starts at the e, not the slash");
    assert_eq!(p[7], Part::Path);
}

#[test]
fn a_plain_argument_stays_plain() {
    let p = parts_of("echo hello");
    assert_eq!(p[5], Part::Plain);
}

/// After a pipe the line starts again, so the next word is a command rather
/// than an argument of the one before.
#[test]
fn a_word_after_an_operator_is_a_command_again() {
    let line = "cat f | grep x";
    let p = parts_of(line);
    let pipe = line.find('|').unwrap();
    assert_eq!(p[pipe], Part::Operator);
    let grep = line.find("grep").unwrap();
    assert_eq!(p[grep], Part::Command);
}
