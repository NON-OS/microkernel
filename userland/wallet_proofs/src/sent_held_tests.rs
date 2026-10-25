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

//! What waiting payments hold back, what is trimmed, and a broadcast that broke off.

use super::*;

#[test]
fn waiting_payments_hold_back_what_they_may_spend() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    l.record(sent(MAINNET, 5, 2));
    assert_eq!(l.held(MAINNET, &ME, NOX), (200, 10));
    assert_eq!(l.held(MAINNET, &ME, ETH), (200, 0));
    l.judge(&[1; 32], reading(Some((100, true)), 100, 5, 6), 2_000);
    assert_eq!(l.held(MAINNET, &ME, NOX), (100, 5), "one in a block is in the balance");
    assert_eq!(l.held(SEPOLIA, &ME, NOX), (0, 0));
}

#[test]
fn settled_payments_are_trimmed_and_waiting_ones_never() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 0, 200));
    for i in 1..40u8 {
        let mut s = sent(MAINNET, u64::from(i), i);
        s.fate = Fate::Confirmed { ok: true };
        l.record(s);
    }
    assert!(l.list.len() <= 17);
    assert!(l.get(&[200; 32]).is_some(), "the waiting one is kept");
}

#[test]
fn a_broadcast_that_broke_off_holds_the_next_until_settled_or_late() {
    let mut l = Ledger::default();
    let mut s = sent(MAINNET, 4, 1);
    s.unknown = true;
    l.record(s);
    assert!(l.unknown_waiting(MAINNET, &ME, 2_000, 600_000));
    /* Kept across a switch: back on mainnet it still holds. */
    assert!(!l.unknown_waiting(SEPOLIA, &ME, 2_000, 600_000));
    assert!(!l.unknown_waiting(MAINNET, &ME, 1_000 + 600_000, 600_000));
    l.judge(&[1; 32], reading(Some((100, true)), 100, 5, 5), 2_000);
    assert!(!l.unknown_waiting(MAINNET, &ME, 2_000, 600_000), "in a block, it landed");
}
