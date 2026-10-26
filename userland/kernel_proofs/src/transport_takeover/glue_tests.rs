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
//! over in one hold of HOLDERS. The rest is pinned whole in glue_whole_tests.

const LOCK: &str = include_str!("../../../../src/services/lifecycle/transport_lock.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing `{needle}`"))
}

#[test]
fn the_lock_is_taken_and_taken_over_inside_one_hold() {
    let hold = &LOCK[at(LOCK, "let mut holders = HOLDERS.lock();")..];
    let hold = &hold[..at(hold, "super::reply_rest::rest(round);")];
    let (take, over) = (at(hold, "lock.try_lock()"), at(hold, "lock.force_unlock()"));
    let flat: String = hold[take..].split_whitespace().collect();
    let cond = ".any(|&(k,pid)|k==key&&super::transport_holder::never_runs_again(pid)){//SAFETY";
    assert!(take < over && flat.contains(cond), "the takeover not gated by the rule alone");
}
