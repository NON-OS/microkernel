//! The lander as a Tor onion service. The onion address authenticates it, so requests are plain
//! HTTP inside Tor. A proof pays whoever submits its settlement, so the lander holds no role but
//! landing: anyone may land the same proof first, and a lander that does not take the submitter
//! fee is not used.

mod base64;
#[cfg(feature = "fuzzing")]
pub(crate) mod fuzz_hook;
mod landers;
mod open;
mod reply;

use crate::error::NetError;
use crate::net::rpc::exchange;
use crate::net::tor::Tor;

pub(crate) use base64::encode;
pub use landers::hand_first;
pub use reply::{Handed, RelayState};

fn ask(
    tor: &Tor,
    onion: &str,
    (method, path): (&str, &str),
    body: Option<&str>,
) -> Result<(u16, String), NetError> {
    let stream = open::open(tor, onion)?;
    let (status, bytes) = exchange(stream, onion, method, path, body)?;
    Ok((status, String::from_utf8(bytes).map_err(|_| NetError::ReplyShape)?))
}

pub fn hand(tor: &Tor, onion: &str, json: &str) -> Result<Handed, NetError> {
    let (status, body) = ask(tor, onion, ("POST", "/v1/handoff"), Some(json))?;
    reply::handed(status, &body)
}

pub fn state(tor: &Tor, onion: &str, id: &str) -> Result<RelayState, NetError> {
    if !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_') || id.is_empty() {
        return Err(NetError::ReplyShape);
    }
    let path = format!("/v1/handoff/{id}");
    let (status, body) = open::read(|| ask(tor, onion, ("GET", &path), None))?;
    if status != 200 {
        return Err(NetError::Rejected { code: status });
    }
    reply::state(&body)
}
