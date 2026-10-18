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

//! Confirmed, reverted or replaced, from what the chain says reading after reading.

use super::*;

#[test]
fn confirmed_only_under_enough_blocks() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    let h = [1; 32];
    assert_eq!(
        l.judge(&h, reading(Some((100, true)), 100, 5, 5), 2_000),
        Some(Fate::InBlock { block: 100, ok: true })
    );
    assert_eq!(l.judge(&h, reading(Some((100, true)), 100 + CONFIRMATIONS - 2, 5, 5), 3_000), None);
    assert_eq!(
        l.judge(&h, reading(Some((100, true)), 100 + CONFIRMATIONS - 1, 5, 5), 4_000),
        Some(Fate::Confirmed { ok: true })
    );
    /* Settled: nothing more changes it. */
    assert_eq!(l.judge(&h, reading(None, 500, 9, 9), 5_000), None);
}

#[test]
fn a_reverted_payment_is_confirmed_as_reverted() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    let head = 100 + CONFIRMATIONS;
    assert_eq!(
        l.judge(&[1; 32], reading(Some((100, false)), head, 5, 5), 2_000),
        Some(Fate::Confirmed { ok: false })
    );
}

#[test]
fn replaced_only_after_its_nonce_is_used_reading_after_reading() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    let h = [1; 32];
    for _ in 1..REPLACED_AFTER {
        assert_eq!(l.judge(&h, reading(None, 100, 5, 5), 2_000), None);
    }
    assert_eq!(l.judge(&h, reading(None, 100, 5, 5), 2_000), Some(Fate::Replaced));
    /* Its nonce stays used. */
    assert_eq!(l.next_nonce(MAINNET, &ME, 4), Some(5));
}

#[test]
fn a_receipt_between_misses_starts_the_count_again() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    let h = [1; 32];
    l.judge(&h, reading(None, 100, 5, 5), 2_000);
    l.judge(&h, reading(Some((100, true)), 101, 5, 5), 2_000);
    l.judge(&h, reading(None, 102, 5, 5), 2_000);
    assert_eq!(l.get(&h).map(|s| s.misses), Some(1));
    assert!(matches!(l.get(&h).map(|s| s.fate), Some(Fate::Waiting)));
}
