//! A reply cut by an unexpected end: whole when its body is complete, refused when short.

use super::super::http::post;
use crate::error::NetError;
use std::io::{Cursor, Read, Result as IoResult, Write};

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
