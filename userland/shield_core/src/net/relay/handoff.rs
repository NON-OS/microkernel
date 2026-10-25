//! The two requests to a lander: hand it a settlement, and ask a hand-off's state.

use super::{open, reply, Handed, RelayState};
use crate::error::NetError;
use crate::net::rpc::exchange;
use crate::net::tor::Tor;

pub(super) fn ask(
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
