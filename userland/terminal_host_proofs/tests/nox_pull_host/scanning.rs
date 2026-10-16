// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Byte scanning helpers shared by the parsers.

use crate::scan;

#[test]
fn find_locates_crlf_gap() {
    let b = b"HTTP/1.1 200 OK\r\nX: y\r\n\r\nBODY";
    assert_eq!(scan::find(b, b"\r\n\r\n"), Some(21));
}

#[test]
fn split_lines_strips_cr() {
    let lines = scan::split_lines(b"a\r\nbb\r\nccc");
    assert_eq!(lines, vec![&b"a"[..], &b"bb"[..], &b"ccc"[..]]);
}

#[test]
fn parse_usize_rejects_nondigits() {
    assert_eq!(scan::parse_usize(b" 4096 "), Some(4096));
    assert_eq!(scan::parse_usize(b"12x"), None);
}

#[test]
fn eq_ci_is_case_insensitive() {
    assert!(scan::eq_ci(b"Content-Length", b"content-length"));
    assert!(!scan::eq_ci(b"a", b"ab"));
}
