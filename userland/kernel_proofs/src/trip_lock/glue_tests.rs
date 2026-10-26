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

//! The trip lock's kernel glue pinned whole (L81b, G1 to G4 and G6): the
//! look that sweeps before it waits, the wait that reads its mark before the
//! look, the drop that lets the data go before the row, the forget, and the
//! rest that checks no listed lock is held.

use crate::proofs::proofs_8::transport_takeover::text::{between, code, tail};

const LOCK: &str = include_str!("../../../../src/services/lifecycle/trip_lock.rs");
const GUARD: &str = include_str!("../../../../src/services/lifecycle/trip_guard.rs");
const BACK: &str = include_str!("../../../../src/services/lifecycle/trip_back.rs");
const REST: &str = include_str!("../../../../src/services/lifecycle/reply_rest.rs");

#[test]
fn g1_the_look_sweeps_the_gone_before_it_waits() {
    let look = concat!(
        "fn look(&self, me: u32, whole: bool) -> bool { let _quiet = super::rest_mark::quiet();",
        " let (mut rows, now) = (self.rows.lock(), crate::time::timestamp_millis());",
        " let gone = super::transport_holder::never_runs_again;",
        " if !rows.may_take(me, whole, now) && rows.sweep(gone) { self.left(true); }",
        " let take = rows.may_take(me, whole, now); if take { rows.take(me, whole); } else {",
        " super::trip_back::asked_again(me, rows.has(me)); rows.wait(me, whole, now); } take } }",
    );
    assert_eq!(tail(LOCK, "fn look("), code(look));
}

#[test]
fn g2_the_wait_reads_its_mark_before_it_looks() {
    let wait = concat!(
        "loop { let mark = super::reply_wait::wake_mark(); if self.look(me, whole) { return me; }",
        " super::reply_rest::rest_since(round, mark); round = round.saturating_add(1); }",
    );
    assert_eq!(between(LOCK, "loop {", "fn look("), code(wait) + "}");
}

#[test]
fn g3_the_writer_lets_go_before_its_row_and_wakes_after_it() {
    let drop = concat!(
        "fn drop(&mut self) { if WHOLE { self.lock.left(false); }",
        " let (had, woken) = self.lock.rows.lock().give(self.me, WHOLE);",
        " super::trip_back::gave(self.me, had);",
        " woken.into_iter().for_each(crate::sched::wake_process); } }",
    );
    assert_eq!(tail(GUARD, "fn drop("), code(drop));
}

#[test]
fn g4_a_reused_pid_is_forgotten_with_its_writer_put_right() {
    let forget = concat!(
        "pub fn forget(&self, pid: u32) { let _quiet = super::rest_mark::quiet();",
        " let mut rows = self.rows.lock(); if rows.sweep(|p| p == pid) { self.left(true); } } }",
    );
    assert_eq!(between(BACK, "pub fn forget(", "pub(super) fn asked_again("), code(forget));
}

#[test]
fn g6_every_rest_first_checks_no_listed_lock_is_held() {
    let first = "pub(super) fn rest_since(round: u32, mark: u64) { super::rest_mark::check();";
    assert!(code(REST).contains(&code(first)), "a rest that does not check first");
}
