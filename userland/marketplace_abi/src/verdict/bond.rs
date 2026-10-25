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

//! The install_ready bond gate (plan/25, B6): a listing that sells needs an
//! active bond of at least the market minimum, read through settle no more
//! than an hour ago (plan/23), whose payee and publisher key are the
//! entry's own. In escrow mode the price is at most the bond's amount, the
//! most a lost dispute can slash.

use super::bond_reading::BondReading;
use crate::types::{MarketplaceEntry, SettleMode};

/// The oldest bond reading the market trusts, in seconds.
pub const BOND_MAX_AGE: u64 = 3_600;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BondVerdict {
    /// Free, or no commerce block: no bond is asked for.
    NotNeeded,
    Active,
    /// No reading, or one older than BOND_MAX_AGE (or from the future).
    Unread,
    /// Another payee's or another publisher key's bond.
    NotThisListing,
    /// Not active, or unbonding.
    Inactive,
    BelowMinimum,
    /// Escrow at a price above what the bond can cover.
    PriceAboveBond,
}

pub fn bond_verdict(
    entry: &MarketplaceEntry,
    r: Option<&BondReading>,
    min: u128,
    now: u64,
) -> BondVerdict {
    let Some(c) = entry.commerce.as_ref().filter(|c| c.settle_mode != SettleMode::Free) else {
        return BondVerdict::NotNeeded;
    };
    let Some(r) = r.filter(|r| r.read_at <= now && now - r.read_at <= BOND_MAX_AGE) else {
        return BondVerdict::Unread;
    };
    if r.payee != c.payee || r.publisher_key != entry.publisher_pubkey {
        BondVerdict::NotThisListing
    } else if !r.active || r.unlock_at != 0 {
        BondVerdict::Inactive
    } else if r.amount < min {
        BondVerdict::BelowMinimum
    } else if c.settle_mode == SettleMode::Escrow && c.unit_price > r.amount {
        BondVerdict::PriceAboveBond
    } else {
        BondVerdict::Active
    }
}
