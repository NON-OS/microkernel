// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Response head parsing.

use crate::http;
use crate::support::some_val;

#[test]
fn http_parse_content_length() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nBODY";
    let p = some_val(http::parse_head(raw));
    assert_eq!(p.status, 200);
    assert_eq!(p.content_length, Some(4));
    assert!(!p.chunked);
    assert_eq!(&raw[p.body_off..], b"BODY");
}

#[test]
fn http_parse_404_and_chunked() {
    let p = some_val(http::parse_head(b"HTTP/1.1 404 Not Found\r\n\r\n"));
    assert_eq!(p.status, 404);
    let c = some_val(http::parse_head(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n"));
    assert!(c.chunked);
}
