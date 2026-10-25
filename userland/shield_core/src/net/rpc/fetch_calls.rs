//! View reads of a contract over the wallet's own Tor: one call, a batch, a batch at one block.

use super::super::http::post;
use super::super::request;
use super::super::response;
use super::text;
use crate::error::NetError;
use crate::net::tor::{tls, Purpose, Tor};

/// Read a view function of `to` over the wallet's own Tor, on the circuits of
/// `purpose`, TLS end to end.
pub fn call_over_tor(
    tor: &Tor,
    purpose: Purpose,
    host: &str,
    to: &str,
    data: &[u8],
) -> Result<Vec<u8>, NetError> {
    let stream = tls(tor.connect(purpose, host, 443)?, host)?;
    let reply = post(stream, host, "/", &request::eth_call(to, data))?;
    response::call_result(text(&reply)?)
}

/// Several view reads of `to` in one batched request over the wallet's own Tor.
pub fn calls_over_tor(
    tor: &Tor,
    purpose: Purpose,
    host: &str,
    to: &str,
    datas: &[Vec<u8>],
) -> Result<Vec<Vec<u8>>, NetError> {
    let stream = tls(tor.connect(purpose, host, 443)?, host)?;
    let reply = post(stream, host, "/", &request::eth_call_batch(to, datas))?;
    response::call_results(text(&reply)?, datas.len())
}

/// Batched view reads at one block over the wallet's own Tor.
pub fn calls_at_over_tor(
    tor: &Tor,
    purpose: Purpose,
    host: &str,
    to: &str,
    datas: &[Vec<u8>],
    block: u64,
) -> Result<Vec<Vec<u8>>, NetError> {
    let stream = tls(tor.connect(purpose, host, 443)?, host)?;
    let reply = post(stream, host, "/", &request::eth_call_batch_at(to, datas, block))?;
    response::call_results(text(&reply)?, datas.len())
}
