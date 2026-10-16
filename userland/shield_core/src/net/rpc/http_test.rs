use super::http::post;
use crate::error::NetError;
use std::io::{Cursor, Read, Result as IoResult, Write};
#[path = "http_eof_test.rs"]
mod eof;

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
