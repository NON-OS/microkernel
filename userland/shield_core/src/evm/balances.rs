//! The account's ETH, NOX and USDC on one network, read in one batch at the same
//! block, over that network's own Tor circuits.

use super::calls::{balance, call};
use super::network::Network;
use super::nox::{with_address, BALANCE_OF};
use super::rpc::ask;
use crate::error::NetError;
use crate::net::tor::Tor;

/// What the account holds, in each coin's base units.
pub struct Holdings {
    pub eth: u128,
    pub nox: u128,
    pub usdc: u128,
}

pub fn balances(tor: &Tor, network: Network, of: &[u8; 20]) -> Result<Holdings, NetError> {
    let chain = network.chain();
    let token = |t: &[u8; 20]| call(None, t, &with_address(BALANCE_OF, of));
    let calls = [balance(of), token(&chain.nox_token), token(&chain.usdc_token)];
    let answers = ask(tor, &chain, &calls)?;
    let at = |i: usize| answers.get(i).ok_or(NetError::ReplyShape);
    Ok(Holdings { eth: at(0)?.quantity()?, nox: at(1)?.word()?, usdc: at(2)?.word()? })
}
