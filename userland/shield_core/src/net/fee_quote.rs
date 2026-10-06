//! The fee of a spend, split as the policy splits it, read before any proving: the protocol part
//! and the network part, the lowest rung of the gas ladder, with the latest base fee shown beside
//! it. A spend proves these figures, and the policy is asked last, so a quote the pool would refuse
//! is never shown.

use super::asset::Asset;
use super::fee_schedule::{bps, schedule, settlement_check, split, with_asset, RUNG};
use super::fee_schedule::{BPS, SCHEDULE_OF};
use super::pool::RPCS;
use super::rpc::call_over_tor;
use super::tor::{Purpose, Tor};
use crate::error::NetError;

/// One fee in note units of its asset, its rung, and the base fee when it was quoted.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Quote {
    pub protocol: u64,
    pub network: u64,
    pub rung: usize,
    pub base_fee: u128,
}

impl Quote {
    pub fn total(&self) -> Option<u64> {
        self.protocol.checked_add(self.network)
    }
}

/// The quote for a spend of `asset` with `public_amount` leaving the pool, from the policy at
/// `policy`, or none when the policy refuses it.
pub fn quote(
    tor: &Tor,
    policy: &str,
    asset: Asset,
    public_amount: u64,
) -> Result<Option<Quote>, NetError> {
    let base_fee = crate::evm::base_fee::sepolia(tor)?;
    let mut last = NetError::Transport;
    for host in RPCS {
        let read = |data: &[u8]| call_over_tor(tor, Purpose::Scan, host, policy, data);
        let found = read(&with_asset(SCHEDULE_OF, asset.id))
            .and_then(|own| Ok((schedule(&own)?, bps(&read(&BPS)?)?)));
        let (own, withdraw_bps) = match found {
            Ok(found) => found,
            Err(e) => {
                last = e;
                continue;
            }
        };
        let at = RUNG;
        let Some((protocol, network)) = split(own, at, withdraw_bps, public_amount) else {
            return Ok(None);
        };
        let fee = protocol.saturating_add(network);
        return match read(&settlement_check(asset.id, public_amount, fee)) {
            Ok(_) => Ok(Some(Quote { protocol, network, rung: at, base_fee })),
            Err(NetError::ReplyShape) => Ok(None),
            Err(e) => Err(e),
        };
    }
    Err(last)
}
