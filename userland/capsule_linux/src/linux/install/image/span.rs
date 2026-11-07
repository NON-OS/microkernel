//! One chunk asked for by its byte range, and the reply read as exactly
//! that chunk: a 206 whose Content-Range is the range asked, or a 200 when
//! the range is the whole object. Anything else is nothing. Pure, so the
//! host proofs hold it.

use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use super::super::http_reply::framing;

/// The GET for bytes `r` of `path`.
pub fn request(path: &str, host: &str, r: &Range<u64>) -> String {
    let last = r.end.saturating_sub(1);
    alloc::format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: nonos\r\nRange: bytes={}-{last}\r\n\r\n",
        r.start
    )
}

/// The bytes of `raw` when it is bytes `r` of an object of `total` bytes,
/// whole.
pub fn chunk(mut raw: Vec<u8>, r: &Range<u64>, total: u64) -> Option<Vec<u8>> {
    let (end, length) = framing(&raw)?;
    let status = raw[..end].split(|b| *b == b'\n').next()?;
    let ok = match status.split(|b| *b == b' ' || *b == b'\r').nth(1)? {
        b"206" => range(&raw[..end]) == Some((r.start, r.end, total)),
        b"200" => r.start == 0 && r.end == total,
        _ => false,
    };
    let want = usize::try_from(r.end.checked_sub(r.start)?).ok()?;
    if !ok || length != Some(want) || raw.len() != end + want {
        return None;
    }
    raw.drain(..end);
    Some(raw)
}

/// `Content-Range: bytes F-L/T` as (F, L + 1, T).
fn range(head: &[u8]) -> Option<(u64, u64, u64)> {
    const NAME: &[u8] = b"content-range:";
    let line = head
        .split(|b| *b == b'\n')
        .find(|l| l.len() > NAME.len() && l[..NAME.len()].eq_ignore_ascii_case(NAME))?;
    let value = core::str::from_utf8(&line[NAME.len()..]).ok()?.trim();
    let (span, total) = value.strip_prefix("bytes ")?.split_once('/')?;
    let (first, last) = span.split_once('-')?;
    let (first, last, total): (u64, u64, u64) =
        (first.parse().ok()?, last.parse().ok()?, total.parse().ok()?);
    (first <= last && last < total).then_some((first, last + 1, total))
}
