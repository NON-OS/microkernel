// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Splitting keep-alive responses into bodies.

use crate::framing;

#[test]
fn frame_splits_two_concatenated_bodies() {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\nabc");
    buf.extend_from_slice(b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\ndef");
    let f1 = framing::frame(&buf).unwrap().expect("first framed");
    assert_eq!(&buf[f1.body.clone()], b"abc");
    let n1 = f1.body.end;
    let f2 = framing::frame(&buf[n1..]).unwrap().expect("second framed");
    assert_eq!(&buf[n1..][f2.body.clone()], b"def");
}

#[test]
fn frame_returns_none_when_body_incomplete() {
    let partial = b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nab";
    assert!(framing::frame(partial).unwrap().is_none());
}

#[test]
fn frame_rejects_missing_content_length_for_keepalive() {
    let no_cl = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n";
    match framing::frame(no_cl) {
        Err((retryable, _)) => assert!(retryable),
        Ok(_) => panic!("expected keep-alive framing rejection"),
    }
}
