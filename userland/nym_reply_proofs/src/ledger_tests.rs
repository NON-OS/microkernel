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

//! The fragments net.nym holds until their acknowledgements come home: what
//! is resent, when, for whom, and the bounds that keep the record small.

use crate::ack_ledger::{
    first_wait_ms, next_wait_ms, Ledger, Resend, LEDGER_RECIPIENT, MAX_HELD, MAX_HELD_BYTES,
    MAX_SENDS, MAX_WAIT_MS,
};

const EXIT_A: [u8; LEDGER_RECIPIENT] = [0xA1; LEDGER_RECIPIENT];
const EXIT_B: [u8; LEDGER_RECIPIENT] = [0xB2; LEDGER_RECIPIENT];

fn id(set: u32, at: u8) -> [u8; 5] {
    let b = set.to_be_bytes();
    [b[0], b[1], b[2], b[3], at]
}

fn take(ledger: &mut Ledger, now: i64, to: Option<&[u8; LEDGER_RECIPIENT]>) -> Vec<Resend> {
    let mut out = Vec::new();
    ledger.due(now, to, &mut out);
    out
}

#[test]
fn a_fragment_is_resent_only_once_its_wait_has_run_out() {
    let mut ledger = Ledger::new();
    ledger.sent(id(7, 1), EXIT_A, vec![1; 100], 1_000, 3_000, 1);
    assert!(take(&mut ledger, 3_999, Some(&EXIT_A)).is_empty(), "not due before its wait");
    assert_eq!(ledger.len(), 1);
    let due = take(&mut ledger, 4_000, Some(&EXIT_A));
    assert_eq!(due.len(), 1);
    assert_eq!(due[0].frag_id, id(7, 1));
    assert_eq!(due[0].fragment, vec![1; 100], "the plaintext comes back to be sealed anew");
    assert_eq!(due[0].sends, 1);
    assert!(ledger.is_empty(), "out of the ledger until it is sent again");
}

#[test]
fn an_acknowledged_fragment_is_never_resent() {
    let mut ledger = Ledger::new();
    for at in 1..=18 {
        ledger.sent(id(9, at), EXIT_A, vec![at; 2000], 0, 3_000, 1);
    }
    // Fifteen of eighteen come home: the case ek's serial showed.
    for at in 1..=15 {
        assert!(ledger.acked(&id(9, at)));
    }
    assert!(!ledger.acked(&id(9, 1)), "a second ack for the same fragment changes nothing");
    let due = take(&mut ledger, 3_000, Some(&EXIT_A));
    let resent: Vec<u8> = due.iter().map(|r| r.frag_id[4]).collect();
    assert_eq!(resent, vec![16, 17, 18], "exactly the three that never arrived");
    assert_eq!(ledger.bytes(), 0);
}

#[test]
fn a_resend_goes_back_in_with_its_count_and_a_longer_wait() {
    let mut ledger = Ledger::new();
    ledger.sent(id(1, 1), EXIT_A, vec![5; 10], 0, 3_000, 1);
    let r = take(&mut ledger, 3_000, Some(&EXIT_A)).pop().unwrap();
    let wait = next_wait_ms(r.wait_ms);
    assert_eq!(wait, 6_000);
    ledger.sent(r.frag_id, EXIT_A, r.fragment, 3_000, wait, r.sends + 1);
    assert!(take(&mut ledger, 8_999, Some(&EXIT_A)).is_empty());
    let r = take(&mut ledger, 9_000, Some(&EXIT_A)).pop().unwrap();
    assert_eq!(r.sends, 2);
    assert_eq!(ledger.next_due(), None);
}

#[test]
fn a_fragment_out_of_tries_is_given_up_not_resent() {
    let mut ledger = Ledger::new();
    ledger.sent(id(2, 1), EXIT_A, vec![0; 8], 0, 1_000, MAX_SENDS);
    let mut out = Vec::new();
    let swept = ledger.due(1_000, Some(&EXIT_A), &mut out);
    assert!(out.is_empty());
    assert_eq!(swept.given_up, 1);
    assert!(ledger.is_empty());
}

#[test]
fn fragments_for_an_exit_no_longer_in_use_are_dropped_at_once() {
    let mut ledger = Ledger::new();
    ledger.sent(id(3, 1), EXIT_A, vec![0; 8], 0, 10_000, 1);
    ledger.sent(id(4, 1), EXIT_B, vec![0; 8], 0, 10_000, 1);
    let mut out = Vec::new();
    // Long before either is due: a rotation must not leave the old exit's
    // fragments to be resent to the new one.
    let swept = ledger.due(1, Some(&EXIT_B), &mut out);
    assert!(out.is_empty());
    assert_eq!(swept.stale, 1);
    assert_eq!(ledger.len(), 1);
    let swept = ledger.due(1, None, &mut out);
    assert_eq!(swept.stale, 1, "no session left: nothing held is for anyone");
    assert!(ledger.is_empty());
}

#[test]
fn the_same_fragment_sent_again_replaces_its_record() {
    let mut ledger = Ledger::new();
    ledger.sent(id(5, 1), EXIT_A, vec![0; 40], 0, 3_000, 1);
    ledger.sent(id(5, 1), EXIT_A, vec![0; 40], 10, 3_000, 2);
    assert_eq!(ledger.len(), 1);
    assert_eq!(ledger.bytes(), 40);
}

#[test]
fn the_ledger_never_grows_past_its_bounds() {
    let mut ledger = Ledger::new();
    let mut let_go = 0;
    for n in 0..(MAX_HELD as u32 + 40) {
        let_go += ledger.sent(id(n, 1), EXIT_A, vec![0; 100], n as i64, 3_000, 1);
    }
    assert_eq!(ledger.len(), MAX_HELD);
    assert_eq!(let_go, 40);
    assert!(!ledger.acked(&id(0, 1)), "the oldest went first");
    assert!(ledger.acked(&id(MAX_HELD as u32 + 39, 1)), "the newest is kept");

    let mut ledger = Ledger::new();
    for n in 0..64u32 {
        ledger.sent(id(n, 1), EXIT_A, vec![0; 32 * 1024], 0, 3_000, 1);
    }
    assert!(ledger.bytes() <= MAX_HELD_BYTES);
    assert_eq!(ledger.bytes(), ledger.len() * 32 * 1024);
}

#[test]
fn the_wait_follows_the_reference_client_within_this_capsules_floor_and_cap() {
    // (out + home) * 1.5 + 1.5 s, as the reference client times an ack.
    assert_eq!(first_wait_ms(2_000, 2_000), 7_500);
    // A route with almost no delay still waits for the idle tick to read it.
    assert_eq!(first_wait_ms(0, 0), 3_000);
    assert_eq!(first_wait_ms(u64::MAX, u64::MAX), MAX_WAIT_MS);
    assert_eq!(next_wait_ms(MAX_WAIT_MS), MAX_WAIT_MS);
    assert_eq!(next_wait_ms(i64::MAX), MAX_WAIT_MS);
}
