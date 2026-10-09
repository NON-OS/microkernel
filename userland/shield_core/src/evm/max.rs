//! The most of one coin a send or a swap can spend, as the Max button fills
//! it in: every unit of a token, and for ether the balance less the most a
//! swap's network fee can be at the fees of the latest block.

use super::calls::{balance, call, latest_block, priority_fee};
use super::network::Network;
use super::nox::{with_address, BALANCE_OF};
use super::rpc::ask;
use crate::error::NetError;
use crate::net::asset::Coin;
use crate::net::tor::Tor;

/// Gas kept back for a swap: above the most a two-hop swap was measured to
/// use on mainnet, 221,000.
const SWAP_GAS: u128 = 300_000;

pub fn max_spend(tor: &Tor, network: Network, of: &[u8; 20], coin: Coin) -> Result<u128, NetError> {
    let chain = network.chain();
    let mut calls = vec![balance(of), latest_block(), priority_fee()];
    if let Some(token) = chain.token(coin) {
        calls.push(call(None, &token, &with_address(BALANCE_OF, of)));
    }
    let answers = ask(tor, &chain, &calls)?;
    let at = |i: usize| answers.get(i).ok_or(NetError::ReplyShape);
    if chain.token(coin).is_some() {
        return at(3)?.word();
    }
    let ether = at(0)?.quantity()?;
    let fee = super::gas::fees(at(1)?, at(2)?)?.map_or(u128::MAX, |f| f.max_fee);
    Ok(ether.saturating_sub(SWAP_GAS.saturating_mul(fee)))
}
