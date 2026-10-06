//! A deposit into the shield signed by this wallet's own public account, worked out in full before
//! anything is signed: the ETH for the fee, the coin held, and for NOX the exact approval of the
//! pool first when its allowance falls short. Each is simulated as the sender, so one the pool or
//! the token would refuse is refused here and never paid for.

use super::calls::{balance, call, latest_block, nonce, priority_fee};
use super::network::Network;
use super::nox::{with_address, BALANCE_OF};
use super::rpc::ask;
pub use super::shield_order::{ShieldChecked, ShieldOrder, Shielding};
use crate::error::NetError;
use crate::net::tor::Tor;

/// `allowance(address,address)`.
const ALLOWANCE: [u8; 4] = [0xdd, 0x62, 0xed, 0x3e];

pub fn review_shield(
    tor: &Tor,
    from: &[u8; 20],
    order: &ShieldOrder,
    sends: &[(u64, [u8; 32])],
) -> Result<ShieldChecked, NetError> {
    let chain = Network::Sepolia.chain();
    let mut calls = vec![balance(from), nonce(from), latest_block(), priority_fee()];
    if let Some(token) = order.token {
        let mut asked = with_address(ALLOWANCE, from);
        asked.extend_from_slice(&[0u8; 12]);
        asked.extend_from_slice(&order.pool);
        calls.push(call(None, &token, &asked));
        calls.push(call(None, &token, &with_address(BALANCE_OF, from)));
    }
    let answers = ask(tor, &chain, &calls)?;
    let at = |i: usize| answers.get(i).ok_or(NetError::ReplyShape);
    let ether = at(0)?.quantity()?;
    let pending = u64::try_from(at(1)?.quantity()?).map_err(|_| NetError::ReplyShape)?;
    let Some(fees) = super::gas::fees(at(2)?, at(3)?)? else {
        return Ok(ShieldChecked::Refused("Network fees are far above normal. Try again later."));
    };
    if ether == 0 {
        return Ok(ShieldChecked::Refused(
            "This account needs Sepolia ETH to pay the network fee.",
        ));
    }
    let approval = match order.token {
        Some(_) if at(5)?.word()? < order.amount => {
            return Ok(ShieldChecked::Refused("The NOX balance does not cover that."));
        }
        Some(_) => at(4)?.word()? < order.amount,
        None => false,
    };
    super::shield_tx::finish(tor, &chain, from, order, approval, ether, pending, fees, sends)
}
