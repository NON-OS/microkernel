// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! FUNDED (op 21, plan/21): whether one more install of a listing lands on
//! a channel the meter holds, as one bit for app.store. The price is the
//! listing's own, from its commerce record; the caller names a listing and
//! a chain and nothing else, so the bit cannot bisect a deposit.

use nonos_marketplace_abi::{commerce_from_bytes, SettleMode};
use nonos_payment_proto::AccountRow;

use super::super::channel::CHARGE_MARGIN;
use super::record::Offer;

/// The listing's committed unitPrice, if PAY would charge one unit of it on
/// `chain_id` (offer.rs `priced`): the release trusted, the record on that
/// chain and in its NOX, a voucher listing, and one unit a nonzero charge
/// at or above its minimum. Any other record, unreadable included, is None.
pub fn committed(o: &Offer, chain_id: u64, nox: &[u8; 20]) -> Option<u128> {
    let c = commerce_from_bytes(&o.block).ok()?;
    let pays = o.trusted && o.chain_id == chain_id && o.token.as_slice() == nox;
    let one = c.unit_price != 0 && c.unit_price >= c.min_charge;
    (pays && c.settle_mode == SettleMode::Voucher && one).then_some(c.unit_price)
}

/// Whether `r` takes a charge of `price` for `listing` on `chain_id` at
/// `now`, unix s, as Channel::charged would: this listing and chain, not
/// frozen, live past the margin, max(cumulative, settled) plus the price
/// within deposit, checked.
pub fn takes(r: &AccountRow, listing: &[u8; 32], chain_id: u64, price: u128, now: u64) -> bool {
    let after = r.cumulative.max(r.settled).checked_add(price);
    (&r.listing_id, r.chain_id) == (listing, chain_id)
        && !r.frozen
        && now.saturating_add(CHARGE_MARGIN) < r.expiry
        && price != 0
        && after.is_some_and(|c| c <= r.deposit)
}

/// The bit: `o` is the market's record for the listing asked about, `rows`
/// the meter's channels. The channel is keyed by the record's listing id.
pub fn funded<I>(rows: I, o: &Offer, chain_id: u64, nox: &[u8; 20], now: u64) -> bool
where
    I: IntoIterator<Item = AccountRow>,
{
    let Some(price) = committed(o, chain_id, nox) else {
        return false;
    };
    rows.into_iter().any(|r| takes(&r, &o.listing_id, chain_id, price, now))
}
