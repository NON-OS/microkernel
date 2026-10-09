//! Signing a reviewed send and putting it out through the network's send relays.
//! The chain id is asked again just before signing, and a mismatch with the
//! network or the transaction stops the send unsigned. The returned hash must be
//! the hash of the bytes sent, and a refused send is looked up by hash before it
//! is called failed, since another RPC may already have taken it.

use super::calls::{receipt, send_raw};
use super::network::Network;
use super::reply::Answer;
use super::rpc::{ask, ask_via, Call};
use super::tx::Eip1559;
use crate::error::NetError;
use crate::net::rpc::{after, quoted};
use crate::net::tor::Tor;
use k256::ecdsa::SigningKey;

/// Where a sent transaction stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SendStatus {
    Pending,
    Confirmed,
    Failed,
}

pub fn broadcast(
    tor: &Tor,
    network: Network,
    tx: &Eip1559,
    key: &SigningKey,
) -> Result<[u8; 32], NetError> {
    let chain = network.chain();
    if tx.chain_id != chain.chain_id {
        return Err(NetError::WrongChain);
    }
    ask_via(tor, &chain, chain.send_rpcs, &[])?;
    let signed = tx.sign(key).ok_or(NetError::ReplyShape)?;
    let answers = ask_via(tor, &chain, chain.send_rpcs, &[send_raw(&signed.raw)])?;
    match answers.first() {
        Some(Answer::Result(_)) => {
            let hash = answers.first().ok_or(NetError::ReplyShape)?.data()?;
            (hash == signed.hash).then_some(signed.hash).ok_or(NetError::ReplyShape)
        }
        _ if known(tor, network, &signed.hash)? => Ok(signed.hash),
        _ => Err(NetError::Rejected { code: 0 }),
    }
}

/// Whether the network already holds a transaction with this hash.
fn known(tor: &Tor, network: Network, hash: &[u8; 32]) -> Result<bool, NetError> {
    let hex: String = hash.iter().map(|b| format!("{b:02x}")).collect();
    let lookup = Call { method: "eth_getTransactionByHash", params: format!(r#"["0x{hex}"]"#) };
    let chain = network.chain();
    let answers = ask_via(tor, &chain, chain.send_rpcs, &[lookup])?;
    let found = answers.first().ok_or(NetError::ReplyShape)?.result()?;
    Ok(!found.starts_with("null"))
}

/// Where a sent transaction stands, from its receipt.
pub fn status(tor: &Tor, network: Network, hash: &[u8; 32]) -> Result<SendStatus, NetError> {
    let answers = ask(tor, &network.chain(), &[receipt(hash)])?;
    let body = answers.first().ok_or(NetError::ReplyShape)?.result()?;
    if body.starts_with("null") {
        return Ok(SendStatus::Pending);
    }
    match after(body, "\"status\"").and_then(quoted) {
        Some("0x1") => Ok(SendStatus::Confirmed),
        Some("0x0") => Ok(SendStatus::Failed),
        _ => Err(NetError::ReplyShape),
    }
}
