//! SHIELD_FROM_FRESH's review (v1.4.56, v1.4.60): NOX or ETH found at a
//! fresh address this wallet handed out, moved into the Shield from that
//! address as standard deposits. What is below the smallest standard size
//! stays there. NOX is approved for exactly the deposits' sum first, and
//! its gas is the address's own ETH: when that lacks some, the review says
//! how much. ETH pays its own gas: a try that lacks some is tried again with
//! that much less, at most ETH_TRIES times.

use super::super::Wallet;
use super::fresh_batch::Batched;
use super::fresh_part::part;
use super::fresh_view::{said, FreshShield};
use super::view::refused;
use crate::error::WalletError;
use crate::evm::shield_batch::BATCH_ETH_SHORT;
use crate::net::asset::Coin;
use crate::wallet::fresh::{handed_index, ShieldRefusal};

const ETH_TRIES: usize = 3;
const GAS_EATS: &str = "The ETH there barely covers its own gas; it stays at that address.";

impl Wallet {
    /// Review moving `amount` wei of `coin` at `address` into the Shield.
    pub fn review_shield_from_fresh(
        &self,
        address: [u8; 20],
        amount: u128,
        coin: Coin,
    ) -> Result<FreshShield, WalletError> {
        let handed = handed_index(&self.paths().account, &address);
        if handed.is_none() || self.with(|s| s.imported)? {
            return Ok(said(ShieldRefusal::UnknownAddress.why(), 0));
        }
        let tor = self.tor()?;
        let mut room = amount;
        for _ in 0..ETH_TRIES {
            let Some(part) = part(&tor, coin, room)? else {
                return Ok(said(ShieldRefusal::BelowStandard.why(), amount));
            };
            let remainder = amount.checked_sub(part.sum).ok_or(WalletError::Amount)?;
            match (self.fresh_try(&tor, address, &part, remainder)?, coin) {
                (Batched::Done(review), _) => return Ok(review),
                (Batched::Short(lacks), Coin::Eth) => room = room.saturating_sub(lacks),
                (Batched::Short(lacks), _) => {
                    let split = refused(BATCH_ETH_SHORT);
                    return Ok(FreshShield { split, remainder, short_eth: lacks });
                }
            }
        }
        Ok(said(GAS_EATS, amount))
    }
}
