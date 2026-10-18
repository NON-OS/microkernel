// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Unknown flags are rejected.

use crate::ls_flags::parse;

#[test]
fn unknown_flags_are_rejected_not_ignored() {
    for (args, bad) in [
        (vec![&b"ls"[..], &b"-z"[..]], b'z'),
        (vec![&b"ls"[..], &b"-laz"[..]], b'z'),
        (vec![&b"ls"[..], &b"-l"[..], &b"-Q"[..]], b'Q'),
    ] {
        assert_eq!(parse(&args), Err(bad), "expected {} to be rejected", bad as char);
    }
}

#[test]
fn an_unknown_flag_is_never_taken_as_a_path() {
    assert!(parse(&[&b"ls"[..], &b"-lah"[..]]).is_ok());
    assert!(parse(&[&b"ls"[..], &b"--"[..], &b"-lah"[..]]).is_ok());
    assert!(parse(&[&b"ls"[..], &b"-lax"[..]]).is_err());
}
