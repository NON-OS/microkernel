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

//! Unbonding and the Bond's withdrawals: the bond only once bondOf's
//! unlockAt is set and reached (Q6), the credit to this account and only
//! when there is some (Q7).

use nonos_marketplace_abi::SettleMode;

use super::fixture::{entry, K, ME};
use super::intents::word;
use crate::intent::{credit_of, may_withdraw, unbond_of, withdraw_bond_of, Publisher, Why};

fn bond_of(unlock: [u8; 32]) -> [[u8; 32]; 5] {
    [word(0x11), word(0x22), word(1_000), unlock, word(0)]
}

#[test]
fn the_bond_is_withdrawn_only_once_unlocked() {
    assert!(!may_withdraw(&bond_of(word(0)), u64::MAX), "unbonding never asked");
    assert!(!may_withdraw(&bond_of(word(100)), 99), "the delay not yet passed");
    assert!(may_withdraw(&bond_of(word(100)), 100) && may_withdraw(&bond_of(word(100)), 101));
    let mut high = word(100);
    high[23] = 1;
    assert!(!may_withdraw(&bond_of(high), u64::MAX), "unlockAt past uint64");
    assert!(!may_withdraw(&bond_of(word(100))[..4], 200), "four words are no bondOf");
    let e = entry(SettleMode::Escrow);
    let bond_id = e.commerce.as_ref().unwrap().bond_id;
    let out = |b: [[u8; 32]; 5], now| withdraw_bond_of(&K, &e, &ME, &b, now);
    assert_eq!(out(bond_of(word(100)), 100), Ok(vec![Publisher::WithdrawBond { bond_id }]));
    assert_eq!(out(bond_of(word(100)), 99), Err(Why::Locked));
    assert_eq!(out(bond_of(word(0)), 99), Err(Why::Locked));
    assert_eq!(unbond_of(&K, &e, &ME), Ok(vec![Publisher::RequestUnbond { bond_id }]));
}

#[test]
fn the_credit_goes_to_this_account_and_none_is_greyed() {
    let to_me = Ok(vec![Publisher::WithdrawBondCredit { to: ME }]);
    assert_eq!(
        (credit_of(&ME, &word(1)), credit_of(&ME, &word(0))),
        (to_me.clone(), Err(Why::NoCredit))
    );
    let mut high = [0u8; 32];
    high[0] = 1;
    assert_eq!(credit_of(&ME, &high), to_me, "a credit above u128 is still some");
}
