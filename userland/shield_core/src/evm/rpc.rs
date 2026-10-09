//! Talking to one network's RPCs over the wallet's own Tor, TLS end to end.
//!
//! Every batch opens with `eth_chainId`. An RPC answering for another chain
//! stops the batch, which is not retried elsewhere: a server lying about its
//! chain is a reason to stop. A failed connection moves to the next RPC, and
//! when all fail over Tor the call fails. There is no direct connection.

use super::network::Chain;
use super::reply::{batch, Answer};
use crate::error::NetError;
use crate::net::rpc::post;
use crate::net::tor::{tls, Tor};

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

fn once(
    tor: &Tor,
    chain: &Chain,
    host: &str,
    body: &str,
    n: usize,
) -> Result<Vec<Answer>, NetError> {
    let stream = tls(tor.connect(chain.purpose, host, 443)?, host)?;
    let reply = post(stream, host, "/", body)?;
    let text = core::str::from_utf8(&reply).map_err(|_| NetError::ReplyShape)?;
    let mut answers = batch(text, n.checked_add(1).ok_or(NetError::ReplyShape)?)?.into_iter();
    let id = answers.next().ok_or(NetError::ReplyShape)?.quantity()?;
    if id != u128::from(chain.chain_id) {
        return Err(NetError::WrongChain);
    }
    Ok(answers.collect())
}

fn body(calls: &[Call]) -> String {
    let mut items = vec![r#"{"jsonrpc":"2.0","id":0,"method":"eth_chainId","params":[]}"#.into()];
    for (i, call) in calls.iter().enumerate() {
        let id = i.saturating_add(1);
        items.push(format!(
            r#"{{"jsonrpc":"2.0","id":{id},"method":"{}","params":{}}}"#,
            call.method, call.params
        ));
    }
    format!("[{}]", items.join(","))
}

#[cfg(test)]
mod live {
    #![allow(clippy::expect_used)]
    use super::*;
    use crate::evm::Network;

    /// Every listed RPC of both networks, one at a time, over Tor. It needs the
    /// network, so it runs only with `--ignored`.
    #[test]
    #[ignore]
    fn every_listed_rpc_answers_over_tor_for_its_own_chain() {
        let tor = Tor::start(&std::env::temp_dir().join("nox-tor-rpcs")).expect("bootstrap");
        let body = body(&[]);
        let mut failed = Vec::new();
        for network in [Network::Mainnet, Network::Sepolia] {
            let chain = network.chain();
            for host in chain.rpcs.iter().chain(chain.send_rpcs) {
                if let Err(e) = once(&tor, &chain, host, &body, 0) {
                    failed.push(format!("{network:?} {host}: {e:?}"));
                }
            }
        }
        assert!(failed.is_empty(), "RPCs that did not answer over Tor: {failed:?}");
    }
}
