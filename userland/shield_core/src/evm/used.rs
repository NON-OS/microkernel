//! Whether a public account was ever used on a network: a sent transaction, or anything held.
//! A restore asks this of each account in turn, one batch at one block per network.

use super::calls::{balance, call, nonce};
use super::network::Network;
use super::nox::{with_address, BALANCE_OF};
use super::rpc::ask;
use crate::error::NetError;
use crate::net::tor::Tor;

pub fn used(tor: &Tor, network: Network, of: &[u8; 20]) -> Result<bool, NetError> {
    let chain = network.chain();
    let token = |t: &[u8; 20]| call(None, t, &with_address(BALANCE_OF, of));
    let calls = [nonce(of), balance(of), token(&chain.nox_token), token(&chain.usdc_token)];
    let answers = ask(tor, &chain, &calls)?;
    let at = |i: usize| answers.get(i).ok_or(NetError::ReplyShape);
    Ok(at(0)?.quantity()? > 0 || at(1)?.quantity()? > 0 || at(2)?.word()? > 0 || at(3)?.word()? > 0)
}
