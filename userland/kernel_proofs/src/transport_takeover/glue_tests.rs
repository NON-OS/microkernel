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

//! The glue the model stands for, read (L77e): the lock is taken and taken
//! over in one hold of HOLDERS, the takeover asks the rule with the holder's
//! state and CPU, a reused pid gives back what its old holder had, and a
//! dead caller's round trip returns before it pauses again.

const LOCK: &str = include_str!("../../../../src/services/lifecycle/transport_lock.rs");
const HOLDER: &str = include_str!("../../../../src/services/lifecycle/transport_holder.rs");
const TRIP: &str = include_str!("../../../../src/services/lifecycle/transport.rs");
const PURGE: &str = include_str!("../../../../src/process/exit/purge.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

#[test]
fn the_lock_is_taken_and_taken_over_inside_one_hold() {
    let hold = &LOCK[at(LOCK, "let mut holders = HOLDERS.lock();")..];
    let hold = &hold[..at(hold, "super::reply_wait::rest(round);")];
    let (take, over) = (at(hold, "lock.try_lock()"), at(hold, "lock.force_unlock()"));
    let flat: String = hold[take..].split_whitespace().collect();
    let cond = ".any(|&(k,pid)|k==key&&super::transport_holder::never_runs_again(pid)){//SAFETY";
    assert!(take < over && flat.contains(cond), "the takeover not gated by the rule alone");
    let rule = &HOLDER[at(HOLDER, "fn never_runs_again(")..];
    assert!(rule.contains("cpu_on_tables(pid).is_some()") && rule.contains("takes_over(pid,"));
    assert!(PURGE.contains("forget_transport_holder(pid)"), "a reused pid keeps a dead lock");
    let trip = &TRIP[at(TRIP, "for round in 0..RECV_YIELDS")..];
    let killed = at(trip, "selection::is_dead(p)");
    assert!(killed < at(trip, "reply_wait::pause("), "a dead caller pauses on");
    assert!(trip[killed..].contains("if !state.is_alive() || killed {"), "the answer ignored");
}
