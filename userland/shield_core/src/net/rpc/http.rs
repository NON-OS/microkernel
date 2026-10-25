//! HTTP/1.1 over an open stream: one request, one reply, then close. The reply is capped,
//! since it comes from a machine the wallet does not trust to bound it.

use crate::error::NetError;
use std::io::{Read, Write};
use std::time::Duration;

#[path = "http_body.rs"]
mod http_body;
#[path = "http_read.rs"]
mod http_read;
use http_body::body_of;
use http_read::read_whole;

/// The whole reply's time: six of a read's 30 s stall limits, room for a full
/// 16 MiB page over the anonymity network.
const REPLY_WITHIN: Duration = Duration::from_secs(180);

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
    body_of(&read_whole(&mut stream, REPLY_WITHIN)?)
}
