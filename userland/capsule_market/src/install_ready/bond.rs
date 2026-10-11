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

//! The bond gate (plan/25, v1.4.23): a sold listing's bond is active, at
//! least MIN_BOND and, in escrow, at least the price, on the chain the
//! request names, as the market's own cache read it under an hour ago. No
//! chain, or no reading, is "not checked", and that never passes.

use nonos_marketplace_abi::{bond_verdict, BondVerdict, MarketplaceEntry, SettleMode};

use crate::bonds::BondCache;

pub fn bond_gate(
    cache: &mut BondCache,
    entry: &MarketplaceEntry,
    chain: Option<u64>,
    now: u64,
) -> bool {
    let Some(c) = entry.commerce.as_ref().filter(|c| c.settle_mode != SettleMode::Free) else {
        return true;
    };
    let Some(chain) = chain else { return false };
    let got = cache.reading((chain, c.bond_id), now);
    let min = got.map_or(0, |g| g.1);
    bond_verdict(entry, got.map(|g| g.0).as_ref(), min, now) == BondVerdict::Active
}
