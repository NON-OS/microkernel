//! The whole reply read from a peer the wallet does not trust: capped in size, and in time as
//! a whole, since a peer that sends a byte inside each read's stall limit would otherwise hold
//! the read open for as long as it liked.

use crate::error::NetError;
use std::io::Read;
use std::time::{Duration, Instant};

/// A page of logs is large but bounded, so a reply past this is a misbehaving peer.
const MAX_REPLY: usize = 16 * 1024 * 1024;

/// Everything `stream` sends until it ends, within `within`: TooSlow past it, ReplyTooLarge
/// past MAX_REPLY.
pub(super) fn read_whole<S: Read>(stream: &mut S, within: Duration) -> Result<Vec<u8>, NetError> {
    let until = Instant::now().checked_add(within).ok_or(NetError::Transport)?;
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
            return Ok(reply);
        }
        if reply.len().saturating_add(n) > MAX_REPLY {
            return Err(NetError::ReplyTooLarge);
        }
        reply.extend_from_slice(buf.get(..n).unwrap_or(&[]));
        if Instant::now() >= until {
            return Err(NetError::TooSlow);
        }
    }
}

#[cfg(test)]
#[path = "http_read_test.rs"]
mod http_read_test;
