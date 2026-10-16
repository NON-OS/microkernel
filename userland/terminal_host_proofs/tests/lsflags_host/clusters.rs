// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Clustered, separate and repeated flags.

use crate::ls_flags::LsFlags;
use crate::support::run;

#[test]
fn bare_ls_sets_nothing_and_takes_no_operand() {
    let (flags, operands) = run(&[b"ls"]);
    assert_eq!(flags, LsFlags::default());
    assert!(operands.is_empty());
}

/// The reason this exists: `ls -lah` used to be treated as a path.
#[test]
fn clustered_flags_all_apply() {
    let (flags, operands) = run(&[b"ls", b"-lah"]);
    assert!(flags.long && flags.all && flags.human);
    assert!(!flags.recurse && !flags.by_time && !flags.by_size);
    assert!(operands.is_empty());
}

#[test]
fn separate_flags_match_the_clustered_form() {
    assert_eq!(run(&[b"ls", b"-l", b"-a", b"-h"]).0, run(&[b"ls", b"-lah"]).0);
    assert_eq!(run(&[b"ls", b"-1RtS"]).0, run(&[b"ls", b"-1", b"-R", b"-t", b"-S"]).0);
}

#[test]
fn repeated_flags_are_idempotent() {
    assert_eq!(run(&[b"ls", b"-l", b"-ll", b"-l"]).0, run(&[b"ls", b"-l"]).0);
}
