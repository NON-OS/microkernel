// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Option flags and request headers on the command line.

use crate::args;

#[test]
fn args_parse_skip_unchanged_flag() {
    let argv: [&[u8]; 2] = [b"-u", b"example.com/f.txt"];
    let a = args::parse(&argv).expect("parse");
    assert!(a.skip_unchanged);
    assert!(!a.no_clobber);
}

#[test]
fn args_parse_both_flags_any_order() {
    let argv: [&[u8]; 3] = [b"-u", b"-n", b"example.com/f.txt"];
    let a = args::parse(&argv).expect("parse");
    assert!(a.skip_unchanged);
    assert!(a.no_clobber);
}

#[test]
fn args_parse_auth_and_header() {
    let argv: [&[u8]; 5] = [b"-a", b"user:pass", b"-H", b"X-Test: 1", b"example.com/f.txt"];
    let a = args::parse(&argv).expect("parse");
    assert_eq!(a.auth.as_deref(), Some(&b"user:pass"[..]));
    assert_eq!(a.headers.len(), 1);
    assert_eq!(&a.headers[0], b"X-Test: 1");
}

#[test]
fn args_reject_crlf_in_header() {
    let argv: [&[u8]; 3] = [b"-H", b"X: 1\r\nEvil: 2", b"example.com/f.txt"];
    assert!(args::parse(&argv).is_err());
}
