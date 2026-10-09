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

/*
 * The sent-payments ledger: a second payment never takes the first's
 * nonce, nothing is forgotten across a network switch, and each payment
 * ends confirmed, replaced or dropped from what the chain says, never
 * waiting forever.
 */

use crate::sent::{Fate, Ledger, Reading, Sent, CONFIRMATIONS, DROP_MS, REPLACED_AFTER};

const MAINNET: u64 = 1;
const SEPOLIA: u64 = 11_155_111;
const ME: [u8; 20] = [7; 20];
const ETH: u8 = 0;
const NOX: u8 = 1;

fn sent(chain: u64, nonce: u64, tag: u8) -> Sent {
    Sent {
        chain,
        from: ME,
        nonce,
        hash: [tag; 32],
        at_ms: 1_000,
        eth_cost: 100,
        token: Some((NOX, 5)),
        fate: Fate::Waiting,
        misses: 0,
        unknown: false,
    }
}

fn reading(receipt: Option<(u64, bool)>, head: u64, latest: u64, pending: u64) -> Reading {
    Reading { receipt, head, latest, pending }
}

#[test]
fn a_second_payment_never_takes_the_first_ones_nonce() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    /* A load-balanced node that has not seen the first says 4 again. */
    assert_eq!(l.next_nonce(MAINNET, &ME, 4), 5);
    /* A node that is ahead wins. */
    assert_eq!(l.next_nonce(MAINNET, &ME, 9), 9);
    /* Another network and another account are their own. */
    assert_eq!(l.next_nonce(SEPOLIA, &ME, 4), 4);
    assert_eq!(l.next_nonce(MAINNET, &[8; 20], 4), 4);
}

#[test]
fn a_dropped_payment_frees_its_nonce() {
    let mut l = Ledger::default();
    l.record(sent(MAINNET, 4, 1));
    let late = 1_000 + DROP_MS;
    assert_eq!(l.judge(&[1; 32], reading(None, 100, 4, 4), late), Some(Fate::Dropped));
    assert_eq!(l.next_nonce(MAINNET, &ME, 4), 4);
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
    assert_eq!(l.next_nonce(MAINNET, &ME, 4), 5);
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

#[test]
fn a_receipts_block_is_read_from_its_own_field() {
    use crate::wallet::rpc::parse_receipt_block;
    let r = br#"{"id":100,"result":{"blockNumber":"0x15f2a3c","status":"0x1"}}"#;
    assert_eq!(parse_receipt_block(r), Some(0x15f2a3c));
    assert_eq!(parse_receipt_block(br#"{"id":100,"result":null}"#), None);
    assert_eq!(parse_receipt_block(br#"{"result":{"blockNumber":"0x"}}"#), None);
}

/* A window left alone locks after five minutes, never while a payment goes. */
#[test]
fn an_idle_window_locks_itself_but_never_mid_payment() {
    use crate::idle_lock::{due, IDLE_LOCK_MS};
    assert!(!due(IDLE_LOCK_MS - 1, 0, false, false));
    assert!(due(IDLE_LOCK_MS, 0, false, false));
    assert!(!due(IDLE_LOCK_MS * 3, 0, true, false), "already locked");
    assert!(!due(IDLE_LOCK_MS * 3, 0, false, true), "a payment is going out");
}
