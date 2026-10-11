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

//! The bond gate on INSTALL_READY: free passes, a sold listing needs a
//! chain and a fresh cached reading of its own bond (v1.4.23), and the
//! answer names why (v1.4.35).

use nonos_market_proto::BondCode::{self, *};
use nonos_marketplace_abi::{MarketplaceEntry, SettleMode};

use super::words::{reading, CHAIN, MS, NOW};
use crate::abi_v3::fixtures::sold;
use crate::bonds::{BondCache, Row};
use crate::install_ready::bond::bond_gate;

fn entry(mode: SettleMode) -> MarketplaceEntry {
    let mut e = sold().entries[0].clone();
    e.publisher_pubkey = [0x11; 32];
    let c = e.commerce.as_mut().unwrap();
    (c.settle_mode, c.unit_price) = (mode, 4_000);
    e
}

#[test]
fn free_passes_and_a_sold_listing_needs_a_chain() {
    let mut c = BondCache::default();
    assert_eq!(bond_gate(&mut c, &entry(SettleMode::Free), None, NOW), NotNeeded);
    assert_eq!(bond_gate(&mut c, &entry(SettleMode::Voucher), None, NOW), Unread);
    assert_eq!(c.next(MS), None, "nothing to read without a chain");
}

#[test]
fn a_sold_listing_passes_on_its_own_fresh_bond_alone() {
    let e = entry(SettleMode::Escrow);
    let key = (CHAIN, e.commerce.as_ref().unwrap().bond_id);
    let mut c = BondCache::default();
    let gate = |c: &mut BondCache, chain| bond_gate(c, &e, Some(chain), NOW);
    assert_eq!(gate(&mut c, CHAIN), Unread, "a miss fails");
    assert!(c.next(MS).is_some(), "and is queued");
    c.put(Row { key, reading: reading(NOW - 10), min: 1_000 });
    assert_eq!(gate(&mut c, CHAIN), Active);
    assert_eq!(gate(&mut c, 1), Unread, "another chain");
    c.put(Row { key, reading: reading(NOW - 10), min: 6_000 });
    assert_eq!(gate(&mut c, CHAIN), BelowMinimum);
    c.put(Row { key, reading: reading(NOW - 3_601), min: 1_000 });
    assert_eq!(gate(&mut c, CHAIN), Unread, "over an hour old");
    assert!(!BondCode::Unread.passes() && Active.passes() && NotNeeded.passes());
}
