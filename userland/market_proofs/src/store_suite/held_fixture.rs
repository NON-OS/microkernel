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
//! A sold listing and a channel the meter holds for it, for the channel
//! tests; meter_fixture has the market's record and the meter's answer.

use nonos_nox_config::SEPOLIA;
use nonos_payment_proto::AccountRow;

use super::fixture::listing;
use crate::meter::channel::CHARGE_MARGIN;
use crate::store::listing::Listing;
use crate::store::price::Price;

pub const NOW: u64 = 1_800_000_000;
pub const KEY: [u8; 32] = [7; 32];

pub fn sold(mode: u8) -> Listing {
    let mut l = listing(b"linux.qwen-small", b"Qwen");
    let (unit, payee, trusted) = (&b"install"[..], [0; 20], true);
    l.known.price = Some(Price { mode, unit, unit_price: 3, payee, trusted });
    l
}

/// A channel for KEY on Sepolia with room for two installs at 3.
pub fn row() -> AccountRow {
    let (channel_id, listing_id, chain_id, reclaim_to) = ([1; 32], KEY, SEPOLIA, [0; 20]);
    let (deposit, settled, cumulative, expiry) = (10, 1, 4, NOW + 2 * CHARGE_MARGIN);
    let frozen = false;
    AccountRow {
        channel_id,
        listing_id,
        chain_id,
        deposit,
        settled,
        cumulative,
        expiry,
        frozen,
        reclaim_to,
    }
}
