//! The base fee of the latest Sepolia block, read over Tor, for picking the gas rung of a spend.

use super::calls::latest_block;
use super::gas::base_fee;
use super::network::Network;
use super::rpc::ask;
use crate::error::NetError;
use crate::net::tor::Tor;

/// The latest base fee on Sepolia, in wei.
pub fn sepolia(tor: &Tor) -> Result<u128, NetError> {
    let answers = ask(tor, &Network::Sepolia.chain(), &[latest_block()])?;
    base_fee(answers.first().ok_or(NetError::ReplyShape)?.result()?)
}
