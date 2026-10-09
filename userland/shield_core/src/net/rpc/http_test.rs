/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used, clippy::indexing_slicing)]

use super::http::post;
use crate::error::NetError;
use std::io::{Cursor, Read, Result as IoResult, Write};

/// A stream that discards the request and replays a canned reply.
struct Canned(Cursor<Vec<u8>>);

impl Read for Canned {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.0.read(buf)
    }
}
impl Write for Canned {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

fn canned(reply: &str) -> Canned {
    Canned(Cursor::new(reply.as_bytes().to_vec()))
}

#[test]
fn a_plain_reply_gives_its_body() {
    let reply =
        "HTTP/1.1 200 OK\r\nContent-Length: 16\r\nConnection: close\r\n\r\n{\"result\":\"0x1\"}";
    let body = post(canned(reply), "host", "/", "{}").expect("a 200 has a body");
    assert_eq!(body, b"{\"result\":\"0x1\"}");
}

#[test]
fn a_chunked_reply_is_reassembled() {
    // Two chunks, "{\"result\"" then ":\"0x1\"}", then the zero chunk.
    let reply = "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n9\r\n{\"result\"\r\n7\r\n:\"0x1\"}\r\n0\r\n\r\n";
    let body = post(canned(reply), "host", "/", "{}").expect("chunked parses");
    assert_eq!(body, b"{\"result\":\"0x1\"}");
}

#[test]
fn a_non_200_is_refused() {
    let reply = "HTTP/1.1 429 Too Many Requests\r\nConnection: close\r\n\r\nrate limited";
    assert_eq!(
        post(canned(reply), "host", "/", "{}").unwrap_err(),
        NetError::Rejected { code: 429 }
    );
}

/// A stream that ends with UnexpectedEof, the way a TLS peer does when it hangs
/// up without close_notify.
struct EofAfter(Cursor<Vec<u8>>);

impl Read for EofAfter {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        match self.0.read(buf)? {
            0 => Err(std::io::Error::from(std::io::ErrorKind::UnexpectedEof)),
            n => Ok(n),
        }
    }
}
impl Write for EofAfter {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

#[test]
fn a_whole_body_ending_without_close_notify_is_accepted() {
    let reply = "HTTP/1.1 200 OK\r\nContent-Length: 16\r\n\r\n{\"result\":\"0x1\"}";
    let body =
        post(EofAfter(Cursor::new(reply.as_bytes().to_vec())), "h", "/", "{}").expect("whole");
    assert_eq!(body, b"{\"result\":\"0x1\"}");
}

#[test]
fn a_truncated_body_is_refused() {
    // The exit cut the stream: fewer bytes than the length the server declared.
    let reply = "HTTP/1.1 200 OK\r\nContent-Length: 64\r\n\r\n{\"result\":\"0x1\"}";
    let got = post(EofAfter(Cursor::new(reply.as_bytes().to_vec())), "h", "/", "{}");
    assert_eq!(got.unwrap_err(), NetError::ReplyShape);
}
