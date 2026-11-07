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

//! The keyring's lock verifier (`capsule_keyring/src/lock/keeper.rs`) and
//! the gate on its four operations: only the right passphrase passes, wrong
//! entries are counted and held back, a count read back after a restart
//! holds the next attempt, and only the shell and setup are served.

use nonos_lock::backoff::wait_ms;
use nonos_lock::wire::{OP_LOCK_CHECK, OP_LOCK_LOAD, OP_LOCK_SET, OP_LOCK_STATE};
use nonos_lock::Record;

use crate::server::lock_keeper::{Keeper, Outcome};
use crate::server::protocol::callers::callers;
use crate::server::protocol::lock_callers::lock_may_ask;

const SALT: [u8; 16] = [0x5A; 16];
const PASS: &[u8] = b"lantern orchard 42";

fn keeper_with(pass: &[u8]) -> Keeper {
    let mut k = Keeper::new();
    assert_eq!(k.set(b"", pass, &SALT, 0), Outcome::Taken);
    k
}

#[test]
fn nothing_is_checked_before_a_passphrase_is_set() {
    let mut k = Keeper::new();
    assert!(!k.held());
    assert_eq!(k.check(PASS, 0), Outcome::NoRecord);
}

#[test]
fn only_the_right_passphrase_unlocks() {
    let mut k = keeper_with(PASS);
    assert_eq!(k.check(b"lantern orchard 43", 0), Outcome::Wrong);
    assert_eq!(k.check(b"", 0), Outcome::Invalid);
    assert_eq!(k.check(PASS, 0), Outcome::Taken);
    assert_eq!(k.count(0).failures, 0);
}

#[test]
fn wrong_entries_are_counted_and_then_held_back() {
    let mut k = keeper_with(PASS);
    for n in 1..=3 {
        assert_eq!(k.check(b"not it at all", 10), Outcome::Wrong);
        assert_eq!(k.count(10).failures, n);
        assert_eq!(k.count(10).wait_ms, 0);
    }
    assert_eq!(k.check(b"not it at all", 10), Outcome::Wrong);
    assert_eq!(k.count(10).wait_ms as u64, wait_ms(4));
    // While the wait runs nothing is derived, not even the right passphrase.
    assert_eq!(k.check(PASS, 11), Outcome::Wait);
    assert_eq!(k.count(11).failures, 4);
    let open = 10 + wait_ms(4) as i64;
    assert_eq!(k.check(PASS, open), Outcome::Taken);
    assert_eq!(k.count(open).failures, 0);
    assert_eq!(k.record().unwrap().failures, 0);
}

#[test]
fn the_count_rides_in_the_record_and_survives_a_restart() {
    let mut k = keeper_with(PASS);
    // Five wrong entries, each once its wait had run.
    let mut now = 100;
    for _ in 0..5 {
        now += k.count(now).wait_ms as i64;
        assert_eq!(k.check(b"not it at all", now), Outcome::Wrong);
    }
    let kept = k.record().unwrap().encode();
    assert_eq!(Record::decode(&kept).unwrap().failures, 5);
    // A restart: a fresh keyring takes the record back at uptime 50 ms.
    let mut after = Keeper::new();
    assert_eq!(after.load(Record::decode(&kept).unwrap(), 50), Outcome::Taken);
    assert_eq!(after.count(50).failures, 5);
    assert_eq!(after.count(50).wait_ms as u64, wait_ms(5));
    assert_eq!(after.check(PASS, 51), Outcome::Wait);
    assert_eq!(after.check(PASS, 50 + wait_ms(5) as i64), Outcome::Taken);
}

#[test]
fn a_record_held_is_never_replaced_by_one_read_back() {
    let mut k = keeper_with(PASS);
    let mut fresh = Keeper::new();
    let _ = fresh.set(b"", b"some other words", &SALT, 0);
    let older = Record::decode(&fresh.record().unwrap().encode()).unwrap();
    assert_eq!(k.load(older, 0), Outcome::Held);
    assert_eq!(k.check(PASS, 0), Outcome::Taken);
}

#[test]
fn changing_it_needs_the_current_one() {
    let mut k = keeper_with(PASS);
    assert_eq!(k.set(b"", b"brand new words", &SALT, 0), Outcome::Held);
    assert_eq!(k.set(b"guess", b"brand new words", &SALT, 0), Outcome::Wrong);
    assert_eq!(k.count(0).failures, 1);
    assert_eq!(k.set(PASS, b"short", &SALT, 0), Outcome::Invalid);
    assert_eq!(k.set(PASS, b"brand new words", &[1u8; 16], 0), Outcome::Taken);
    assert_eq!(k.check(PASS, 0), Outcome::Wrong);
    assert_eq!(k.check(b"brand new words", 0), Outcome::Taken);
    assert_eq!(k.record().unwrap().salt, [1u8; 16]);
}

#[test]
fn a_first_passphrase_takes_no_current_and_meets_the_rules() {
    let mut k = Keeper::new();
    assert_eq!(k.set(b"anything", PASS, &SALT, 0), Outcome::NoRecord);
    assert_eq!(k.set(b"", b"seven c", &SALT, 0), Outcome::Invalid);
    assert_eq!(k.set(b"", PASS, &SALT, 0), Outcome::Taken);
}

#[test]
fn the_record_holds_the_tag_and_never_the_passphrase() {
    let k = keeper_with(PASS);
    let bytes = k.record().unwrap().encode();
    assert!(!bytes.windows(PASS.len()).any(|w| w == PASS));
    assert!(!bytes.windows(8).any(|w| w == &PASS[..8]));
}

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"desktop_shell" => Some(10),
        b"app.setup_wizard" => Some(11),
        b"app.settings" => Some(12),
        b"app.nonos_wallet" => Some(13),
        _ => None,
    }
}

#[test]
fn only_the_shell_and_setup_reach_the_verifier() {
    for pid in [10, 11, 12, 13, 99, 0] {
        assert_eq!(lock_may_ask(OP_LOCK_SET, pid, registry), pid == 10 || pid == 11, "set {pid}");
        for op in [OP_LOCK_CHECK, OP_LOCK_LOAD, OP_LOCK_STATE] {
            assert_eq!(lock_may_ask(op, pid, registry), pid == 10, "op {op} pid {pid}");
        }
    }
    // No other op is a lock op, and the lock ops are none of the key store's:
    // dispatch gates them here and returns before the key gate.
    for op in 0..64 {
        let lock = [OP_LOCK_SET, OP_LOCK_CHECK, OP_LOCK_LOAD, OP_LOCK_STATE].contains(&op);
        assert_eq!(lock_may_ask(op, 10, registry), lock, "op {op}");
        if lock {
            assert_eq!(callers(op), None);
        }
    }
    // A name that resolves to no process admits no one.
    assert!(!lock_may_ask(OP_LOCK_CHECK, 10, |_| None));
}
