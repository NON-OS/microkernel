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

//! A voucher listing whose channel is open installs on Enter, the wallet's
//! "Use it" answered: the store reads the meter's ACCOUNT rows and installs when one takes one more install as the meter's charge
//! would. Anything less, a meter that did not answer included, posts.

use nonos_payment_proto::AccountRow;

use super::listing::Listing;
use super::price::{Price, VOUCHER};
use super::purchase_said::by_wallet;

#[path = "channel_page.rs"]
mod channel_page;
pub use channel_page::{page, queued};

/// PAY refuses on a channel within a day of its expiry.
pub const CHARGE_MARGIN: u64 = 86_400;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    /// mk_app_install, as for a listing nobody sells.
    Install,
    /// 19/0 POST to the wallet.
    Post,
}

/// Whether Enter on `l` reads the channels first: a trusted voucher listing
/// the wallet would otherwise be handed. Escrow never does.
pub fn reads_channels(l: &Listing) -> bool {
    by_wallet(l) && l.known.price.is_some_and(|p| p.mode == VOUCHER)
}

/// Whether `r` takes one install (one unit at `p`'s price) at `now`, unix
/// s, as Channel::charged would: this listing and chain, not frozen, live
/// past the margin, max(cumulative, settled) plus the price within deposit.
pub fn funded_for(r: &AccountRow, p: &Price, now: u64) -> bool {
    let after = r.cumulative.max(r.settled).checked_add(p.unit_price);
    (r.listing_id, r.chain_id) == (p.listing_key, p.chain_id)
        && !r.frozen
        && now.saturating_add(CHARGE_MARGIN) < r.expiry
        && p.unit_price != 0
        && after.is_some_and(|c| c <= r.deposit)
}

/// Where Enter sends `l`, given the rows read; None (no answer, an error,
/// a reply that does not read) posts.
pub fn route(l: &Listing, rows: Option<&[AccountRow]>, now: u64) -> Route {
    let held = |p: Price| rows.is_some_and(|rs| rs.iter().any(|r| funded_for(r, &p, now)));
    match !by_wallet(l) || l.known.price.filter(|_| reads_channels(l)).is_some_and(held) {
        true => Route::Install,
        false => Route::Post,
    }
}
