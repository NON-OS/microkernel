// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Positional argument parsing: source, destination, rejects.

use crate::args;
use crate::support::ok_val;

#[test]
fn args_parse_file() {
    let a = ok_val(args::parse(&[&b"10.0.2.2:8000/report.pdf"[..], &b"/docs/report.pdf"[..]]));
    assert_eq!(a.target.hostname, b"10.0.2.2");
    assert_eq!(a.target.port, 8000);
    assert_eq!(a.target.host, b"10.0.2.2:8000");
    assert_eq!(a.target.path, b"/report.pdf");
    assert_eq!(a.dest, b"/docs/report.pdf");
    assert!(!a.target.is_dir);
    assert!(!a.no_clobber);
}

#[test]
fn args_parse_dir_trailing_slash() {
    let a = ok_val(args::parse(&[&b"10.0.2.2:8000/photos/"[..], &b"/docs/photos/"[..]]));
    assert!(a.target.is_dir);
    assert_eq!(a.target.path, b"/photos/");
}

#[test]
fn args_reject_bad() {
    assert!(args::parse(&[]).is_err());
    assert!(args::parse(&[&b"a/x"[..], &b"/d"[..], &b"/e"[..]]).is_err());
    assert!(args::parse(&[&b"hostonly"[..]]).is_err());
    assert!(args::parse(&[&b"host:bad/x"[..]]).is_err());
    assert!(args::parse(&[&b"/x"[..]]).is_err());
}

#[test]
fn args_flag_and_default_dest() {
    let a = ok_val(args::parse(&[&b"-n"[..], &b"10.0.2.2:8000/x.txt"[..]]));
    assert!(a.no_clobber);
    assert_eq!(a.dest, b"x.txt");
    let b = ok_val(args::parse(&[&b"10.0.2.2:8000/x.txt"[..], &b"/docs/x"[..]]));
    assert!(!b.no_clobber);
    assert_eq!(b.dest, b"/docs/x");
}
