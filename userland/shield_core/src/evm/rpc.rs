//! Talking to one network's RPCs over the wallet's own Tor, TLS end to end.
//!
//! Every batch opens with `eth_chainId`. An RPC answering for another chain
//! stops the batch, which is not retried elsewhere: a server lying about its
//! chain is a reason to stop. A failed connection moves to the next RPC, and
//! when all fail over Tor the call fails. There is no direct connection.

#[path = "rpc_wire.rs"]
mod wire;

use super::network::Chain;
use super::reply::Answer;
use crate::error::NetError;
use crate::net::tor::Tor;
use wire::{body, once};

/// One JSON-RPC call: its method and its params as a JSON array.
pub(super) struct Call {
    pub method: &'static str,
    pub params: String,
}

/// Ask `calls` of the network's read RPCs in one batch, after its chain id,
/// and return their answers in order.
pub(super) fn ask(tor: &Tor, chain: &Chain, calls: &[Call]) -> Result<Vec<Answer>, NetError> {
    ask_via(tor, chain, chain.rpcs, calls)
}

/// Ask the same batch of `want` different read RPCs, for a value that must
/// not rest on one server's word. Fewer answering is a failure.
pub(super) fn ask_each(
    tor: &Tor,
    chain: &Chain,
    calls: &[Call],
    want: usize,
) -> Result<Vec<Vec<Answer>>, NetError> {
    let body = body(calls);
    let mut out = Vec::new();
    let mut last = NetError::ProxyUnreachable;
    for host in chain.rpcs {
        match once(tor, chain, host, &body, calls.len()) {
            Ok(answers) => out.push(answers),
            Err(NetError::WrongChain) => return Err(NetError::WrongChain),
            Err(e) => last = e,
        }
        if out.len() == want {
            return Ok(out);
        }
    }
    Err(last)
}

/// The same, of `hosts` in turn.
pub(super) fn ask_via(
    tor: &Tor,
    chain: &Chain,
    hosts: &[&str],
    calls: &[Call],
) -> Result<Vec<Answer>, NetError> {
    let body = body(calls);
    let mut last = NetError::ProxyUnreachable;
    for host in hosts {
        match once(tor, chain, host, &body, calls.len()) {
            Ok(answers) => return Ok(answers),
            Err(NetError::WrongChain) => return Err(NetError::WrongChain),
            Err(e) => last = e,
        }
    }
    Err(last)
}

#[cfg(test)]
#[path = "rpc_live_test.rs"]
mod live;
