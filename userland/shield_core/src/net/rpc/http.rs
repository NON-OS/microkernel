//! HTTP/1.1 over an open stream: one request, one reply, then close. The reply is capped,
//! since it comes from a machine the wallet does not trust to bound it.

use crate::error::NetError;
use std::io::{Read, Write};

/// A page of logs is large but bounded, so a reply past this is a misbehaving peer.
const MAX_REPLY: usize = 16 * 1024 * 1024;

/// POST a JSON body and return the reply body. Any status but 200 is refused unparsed.
pub fn post<S: Read + Write>(
    stream: S,
    host: &str,
    path: &str,
    body: &str,
) -> Result<Vec<u8>, NetError> {
    let (status, body) = exchange(stream, host, "POST", path, Some(body))?;
    if status != 200 {
        return Err(NetError::Rejected { code: status });
    }
    Ok(body)
}

/// One request and its whole reply, for callers that read meaning into other statuses.
pub fn exchange<S: Read + Write>(
    mut stream: S,
    host: &str,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<(u16, Vec<u8>), NetError> {
    let request = match body {
        Some(body) => format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nAccept: application/json\r\nConnection: close\r\n\r\n{body}",
            body.len()
        ),
        None => format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}\r\nAccept: application/json\r\nConnection: close\r\n\r\n"
        ),
    };
    stream.write_all(request.as_bytes()).map_err(|_| NetError::Transport)?;
    // Without a flush a plain onion stream holds the request, and it never leaves.
    stream.flush().map_err(|_| NetError::Transport)?;
    let mut reply = Vec::new();
    let mut buf = [0u8; 8192];
    loop {
        // A TLS hang up without close_notify ends the reply, safe because `body_of` then
        // requires a whole body and refuses a truncating exit.
        let n = match stream.read(&mut buf) {
            Ok(n) => n,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => 0,
            Err(_) => return Err(NetError::Transport),
        };
        if n == 0 {
            break;
        }
        if reply.len().saturating_add(n) > MAX_REPLY {
            return Err(NetError::ReplyTooLarge);
        }
        reply.extend_from_slice(buf.get(..n).unwrap_or(&[]));
    }
    body_of(&reply)
}

fn body_of(reply: &[u8]) -> Result<(u16, Vec<u8>), NetError> {
    let split = find(reply, b"\r\n\r\n").ok_or(NetError::ReplyShape)?;
    let head = reply.get(..split).ok_or(NetError::ReplyShape)?;
    // A status the peer chose, a rate limit above all, is reported so the caller can back off.
    let status = head
        .get(9..12)
        .and_then(|c| core::str::from_utf8(c).ok())
        .and_then(|c| c.parse::<u16>().ok())
        .ok_or(NetError::ReplyShape)?;
    let body = reply.get(split.saturating_add(4)..).unwrap_or(&[]);
    if header_says_chunked(head) {
        return Ok((status, dechunk(body)?));
    }
    // A declared length must match: short is truncation, long is junk.
    if let Some(len) = content_length(head) {
        if body.len() != len {
            return Err(NetError::ReplyShape);
        }
    }
    Ok((status, body.to_vec()))
}

fn content_length(head: &[u8]) -> Option<usize> {
    let lower: Vec<u8> = head.iter().map(u8::to_ascii_lowercase).collect();
    let key = b"content-length:";
    let at = find(&lower, key)?.checked_add(key.len())?;
    let rest = lower.get(at..)?;
    let end = find(rest, b"\r\n").unwrap_or(rest.len());
    core::str::from_utf8(rest.get(..end)?).ok()?.trim().parse().ok()
}

fn header_says_chunked(head: &[u8]) -> bool {
    let lower: Vec<u8> = head.iter().map(u8::to_ascii_lowercase).collect();
    find(&lower, b"transfer-encoding: chunked").is_some()
}

/// Reassemble a chunked body: hex length, CRLF, bytes, CRLF, until a zero length chunk.
fn dechunk(body: &[u8]) -> Result<Vec<u8>, NetError> {
    let mut out = Vec::new();
    let mut rest = body;
    loop {
        let line_end = find(rest, b"\r\n").ok_or(NetError::ReplyShape)?;
        let size_hex = rest.get(..line_end).ok_or(NetError::ReplyShape)?;
        let text = core::str::from_utf8(size_hex).map_err(|_| NetError::ReplyShape)?;
        let size = usize::from_str_radix(text.trim(), 16).map_err(|_| NetError::ReplyShape)?;
        if size == 0 {
            break;
        }
        let start = line_end.saturating_add(2);
        let end = start.saturating_add(size);
        out.extend_from_slice(rest.get(start..end).ok_or(NetError::ReplyShape)?);
        rest = rest.get(end.saturating_add(2)..).ok_or(NetError::ReplyShape)?;
    }
    Ok(out)
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}
