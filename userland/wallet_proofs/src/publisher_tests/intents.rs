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

//! A bond from the entry: refused below MIN_BOND or, for escrow, below the
//! unit price (Q1); the approve skipped only when the allowance is exactly
//! the amount, and otherwise exactly the amount, a stale one reset (Q2).

use nonos_marketplace_abi::SettleMode;
use nonos_nox_config::Contract;

use super::fixture::{entry, K, KEY, ME};
use crate::intent::{bond_of, Publisher, Why};

pub(super) fn word(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

fn bond(mode: SettleMode, amount: u128, allowance: [u8; 32]) -> Result<Vec<Publisher>, Why> {
    bond_of(&K, &entry(mode), &ME, amount, 500, &allowance)
}

#[test]
fn a_bond_is_at_least_min_bond_and_an_escrow_bond_the_unit_price() {
    use SettleMode::*;
    assert_eq!(bond(Escrow, 499, word(0)), Err(Why::BelowMinBond), "below MIN_BOND");
    assert_eq!(bond_of(&K, &entry(Voucher), &ME, 0, 0, &word(0)), Err(Why::BelowMinBond));
    assert_eq!(bond(Escrow, 999, word(0)), Err(Why::BelowUnitPrice), "unit_price 1000");
    assert!(bond(Voucher, 999, word(0)).is_ok() && bond(Free, 500, word(0)).is_ok());
    let both = vec![
        Publisher::Approve { spender: Contract::PublisherBond, amount: 1_000 },
        Publisher::Bond { publisher_key: KEY, amount: 1_000 },
    ];
    assert_eq!(bond(Escrow, 1_000, word(0)), Ok(both));
}

#[test]
fn the_approve_is_skipped_only_at_exactly_the_amount() {
    let only = Ok(vec![Publisher::Bond { publisher_key: KEY, amount: 1_000 }]);
    assert_eq!(bond(SettleMode::Escrow, 1_000, word(1_000)), only);
    for stale in [word(0), word(999), word(1_001), word(u128::MAX), [0xFF; 32]] {
        let calls = bond(SettleMode::Escrow, 1_000, stale).unwrap();
        let approve = Publisher::Approve { spender: Contract::PublisherBond, amount: 1_000 };
        assert_eq!(calls.first(), Some(&approve), "a stale allowance is reset: {stale:?}");
    }
    let mut high = word(1_000);
    high[0] = 1;
    assert_eq!(bond(SettleMode::Escrow, 1_000, high).unwrap().len(), 2, "above u128");
}
