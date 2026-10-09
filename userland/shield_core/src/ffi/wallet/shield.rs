//! Shielding from the public account: the pool read and checked as for any deposit, then the
//! approval or the deposit reviewed and held under an id, confirmed through the same call as a
//! send. The note travels with the held deposit and is stored before the deposit leaves.

use super::shield_view::{refused, shown};
use super::ticket::{approve_calldata, sentence};
use super::{Pending, Wallet};
use crate::error::{NetError, WalletError};
use crate::evm::shield::{review_shield, ShieldChecked, ShieldOrder};
use crate::evm::{self, Network};
use crate::ffi::amount_read::parse_base_units;
use crate::ffi::evm::parse_evm_address;
use crate::ffi::PublicShield;
use crate::net::asset::Coin;
use crate::net::pool::{ACTIVE, RPCS};
use crate::wallet::{check_deposit, prepare_deposit, read_pool_state_checked};
use std::time::Instant;

impl Wallet {
    /// Work out a deposit of `amount` of `coin` from the active account into the shield on Sepolia.
    pub fn review_shield(&self, coin: Coin, amount: String) -> Result<PublicShield, WalletError> {
        let asset = ACTIVE.asset(coin).ok_or(WalletError::Amount)?;
        let amount = parse_base_units(&amount)?;
        // A deposit is public, so it is a standard size inside the pool's range, whole note units.
        let units = u64::try_from(amount.checked_div(asset.scale).unwrap_or(0)).unwrap_or(0);
        if amount.checked_rem(asset.scale) != Some(0) || !asset.allows(units) {
            return Err(WalletError::NotStandard);
        }
        let (from, to) = self.with(|s| (s.evm().address(), s.address()))?;
        let tor = self.tor()?;
        let state = read_pool_state_checked(&tor, &RPCS, &ACTIVE, &from, asset.id)?;
        if state.scale != asset.scale {
            return Ok(refused(
                amount,
                "This pool counts this coin differently. Update the app first.",
            ));
        }
        if let Err(refusal) = check_deposit(&state, amount) {
            return Ok(refused(amount, sentence(refusal)));
        }
        let request = prepare_deposit(&to, amount, asset.id, state.fee_bps, state.scale)?;
        let token = asset.token.map(parse_evm_address).transpose()?;
        let order = ShieldOrder {
            pool: parse_evm_address(ACTIVE.address)?,
            token,
            amount,
            approve_data: &approve_calldata(amount)?,
            deposit_data: &request.calldata,
            refused_as: "The pool would not take this deposit now. Nothing was sent.",
        };
        let sends = evm::nonce::recorded(&self.paths().account, Network::Sepolia, &from);
        let review = match review_shield(&tor, &from, &order, &sends)? {
            ShieldChecked::Ready(review) => review,
            ShieldChecked::Refused(why) => return Ok(refused(amount, why)),
        };
        let id = self.next_review_id();
        let note = (!review.approval).then(|| request.note.clone());
        let shielded = u128::from(request.note.value).saturating_mul(asset.scale);
        let view = shown(id, &review, [amount, request.fee, shielded]);
        let pending = Pending {
            id,
            network: Network::Sepolia,
            from,
            tx: review.tx,
            made: Instant::now(),
            note,
        };
        *self.pending.lock().map_err(|_| NetError::Transport)? = Some(pending);
        Ok(view)
    }
}
