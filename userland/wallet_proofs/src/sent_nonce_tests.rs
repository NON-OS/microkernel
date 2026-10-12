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

//! The next nonce: never one already sent, unless that one was dropped.

use super::*;

#[test]
fn a_second_payment_never_takes_the_first_ones_nonce() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    /* A load-balanced node that has not seen the first says 4 again. */
    assert_eq!(l.next_nonce(MAINNET, &ME, 4), Some(5));
    /* A node that is ahead wins. */
    assert_eq!(l.next_nonce(MAINNET, &ME, 9), Some(9));
    /* Another network and another account are their own. */
    assert_eq!(l.next_nonce(SEPOLIA, &ME, 4), Some(4));
    assert_eq!(l.next_nonce(MAINNET, &[8; 20], 4), Some(4));
}

#[test]
fn a_dropped_payment_frees_its_nonce() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    let late = 1_000 + DROP_MS;
    assert_eq!(l.judge(&[1; 32], reading(None, 100, 4, 4), late), Some(Fate::Dropped));
    assert_eq!(l.next_nonce(MAINNET, &ME, 4), Some(4));
    assert!(l.open(MAINNET, &ME).is_empty());
}

#[test]
fn a_payment_the_node_still_holds_is_not_dropped() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    let late = 1_000 + DROP_MS;
    assert_eq!(l.judge(&[1; 32], reading(None, 100, 4, 5), late), None);
    assert_eq!(l.judge(&[1; 32], reading(None, 100, 4, 4), 2_000), None, "too soon");
}

#[test]
fn a_nonce_is_never_repeated_by_saturating_or_taken_at_u64_max() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, u64::MAX - 2, 1));
    assert_eq!(
        l.next_nonce(MAINNET, &ME, 0),
        Some(u64::MAX - 1),
        "the last a transaction may carry"
    );
    l.record(sent(MAINNET, u64::MAX - 1, 2));
    assert_eq!(l.next_nonce(MAINNET, &ME, 0), None, "u64::MAX is no nonce (EIP-2681)");
    /* A record at u64::MAX, from a hostile node's count, gives no next: it never saturates back onto itself. */
    let mut hostile = Ledger::default();
    hostile.record(sent(MAINNET, u64::MAX, 3));
    assert_eq!(hostile.next_nonce(MAINNET, &ME, 0), None);
    assert_eq!(
        Ledger::default().next_nonce(MAINNET, &ME, u64::MAX),
        None,
        "nor a pending count of u64::MAX"
    );
}
