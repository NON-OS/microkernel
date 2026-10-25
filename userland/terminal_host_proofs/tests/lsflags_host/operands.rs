// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! `--`, lone dashes and plain words as operands.

use crate::ls_flags::LsFlags;
use crate::support::run;

#[test]
fn double_dash_ends_the_flag_run() {
    let (flags, operands) = run(&[b"ls", b"-l", b"--", b"-a", b"--", b"-lah"]);
    assert!(flags.long && !flags.all);
    assert_eq!(operands, vec![&b"-a"[..], &b"--"[..], &b"-lah"[..]]);
}

#[test]
fn every_non_flag_word_is_an_operand() {
    let (flags, operands) = run(&[b"ls", b"/a", b"-l", b"/b", b"/c"]);
    assert!(flags.long);
    assert_eq!(operands, vec![&b"/a"[..], &b"/b"[..], &b"/c"[..]]);
}

/// A lone dash is a path operand, not an empty flag cluster.
#[test]
fn lone_dash_is_an_operand() {
    let (flags, operands) = run(&[b"ls", b"-"]);
    assert_eq!(flags, LsFlags::default());
    assert_eq!(operands, vec![&b"-"[..]]);
}
