// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Request head construction for GET, HEAD and PUT.

use crate::http;
use crate::scan;

#[test]
fn http_build_get_shape() {
    let r = http::build_get(b"10.0.2.2:8000", b"/a.txt", b"");
    assert!(scan::find(&r, b"GET /a.txt HTTP/1.1\r\n").is_some());
    assert!(scan::find(&r, b"Host: 10.0.2.2:8000\r\n").is_some());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn http_build_get_ka_is_keep_alive() {
    let r = http::build_get_ka(b"example.com", b"/f.txt", b"");
    assert!(scan::find(&r, b"Connection: keep-alive").is_some());
    assert!(scan::find(&r, b"Accept-Encoding: gzip").is_some());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn http_build_head_is_head_without_encoding() {
    let r = http::build_head(b"example.com", b"/f.txt", b"");
    assert!(r.starts_with(b"HEAD /f.txt HTTP/1.1"));
    assert!(scan::find(&r, b"Connection: keep-alive").is_some());
    assert!(scan::find(&r, b"Accept-Encoding").is_none());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn http_build_put_head_has_content_length() {
    let r = http::build_put_head(b"h.local", b"/up/x.bin", 42, b"");
    assert!(r.starts_with(b"PUT /up/x.bin HTTP/1.1"));
    assert!(scan::find(&r, b"Host: h.local").is_some());
    assert!(scan::find(&r, b"Content-Length: 42").is_some());
    assert!(scan::find(&r, b"Accept-Encoding").is_none());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn http_build_put_head_includes_extra_headers() {
    let r = http::build_put_head(b"h", b"/p", 0, b"Authorization: Basic abc\r\n");
    assert!(scan::find(&r, b"Authorization: Basic abc").is_some());
    assert!(r.ends_with(b"\r\n\r\n"));
}

#[test]
fn http_build_get_includes_extra_headers() {
    let r = http::build_get(b"h", b"/p", b"Authorization: Basic abc\r\n");
    assert!(scan::find(&r, b"Authorization: Basic abc").is_some());
    assert!(r.ends_with(b"\r\n\r\n"));
}
