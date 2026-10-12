//! A whole HTTP reply read for its status and body: a declared length must match, and a
//! chunked body is reassembled to its zero length chunk.

use crate::error::NetError;

pub(super) fn body_of(reply: &[u8]) -> Result<(u16, Vec<u8>), NetError> {
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
