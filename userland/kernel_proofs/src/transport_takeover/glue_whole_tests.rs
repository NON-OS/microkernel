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

//! The takeover's glue pinned whole, comments and whitespace aside (L81):
//! the rule asked with the holder's own end and CPU in that order, a reused
//! pid's lock unlocked before its rows go, and a killed caller's round trip
//! answering a transport failure, not a dead capsule, before it pauses.

const HOLDER: &str = include_str!("../../../../src/services/lifecycle/transport_holder.rs");
const TRIP: &str = include_str!("../../../../src/services/lifecycle/transport.rs");
const PURGE: &str = include_str!("../../../../src/process/exit/purge.rs");

use super::text::{between, code};

#[test]
fn the_rule_is_asked_with_the_holders_own_end_and_cpu() {
    let rule = concat!(
        "pub(crate) fn never_runs_again(pid: u32) -> bool {",
        " use crate::process::core::{ProcessState, PROCESS_TABLE}; let dead =",
        " |s: &ProcessState| matches!(s, ProcessState::Zombie(_) | ProcessState::Terminated(_));",
        " let ended = PROCESS_TABLE.find_by_pid(pid).is_none_or(|p| dead(&p.state.lock()));",
        " let on_cpu = crate::process::scheduler::selection::cpu_on_tables(pid).is_some();",
        " super::transport_rule::takes_over(pid, ended, on_cpu) }",
    );
    let got = between(HOLDER, "pub(crate) fn never_runs_again(", "pub(crate) fn forget(");
    assert_eq!(got, code(rule), "the rule asked with another end or CPU");
}

#[test]
fn a_reused_pid_unlocks_its_old_holders_lock_before_its_rows_go() {
    let forget = concat!(
        "pub(crate) fn forget(pid: u32) { let _quiet = super::rest_mark::quiet();",
        " let mut holders = HOLDERS.lock();",
        " for &(key, _) in holders.iter().filter(|h| h.1 == pid) {",
        " let lock = unsafe { &*(key as *const Mutex<()>) };",
        " if lock.is_locked() { unsafe { lock.force_unlock() }; } }",
        " holders.retain(|h| h.1 != pid); }",
    );
    let c = code(HOLDER);
    assert_eq!(&c[c.find(&code("pub(crate) fn forget(")).expect("forget")..], code(forget));
    let call = "crate::services::lifecycle::forget_transport_holder(pid);";
    assert!(code(PURGE).contains(&code(call)), "a reused pid keeps a dead lock");
}

#[test]
fn a_killed_caller_returns_a_transport_failure_before_it_pauses() {
    let gate = concat!(
        "for round in 0..RECV_YIELDS { let killed = crate::process::current_pid()",
        " .is_some_and(|p| crate::process::scheduler::selection::is_dead(p));",
        " if killed { return Err(TransportError::TransportFailure); }",
        " if !state.is_alive() { return Err(TransportError::Dead); }",
    );
    let after = "if state.generation() != gen_at_send";
    let got = between(TRIP, "for round in 0..RECV_YIELDS {", after);
    assert_eq!(got, code(gate), "a killed caller said the capsule is dead, or paused on");
}
