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

//! A wake passed on by a waiter returning early skips one killed but not yet
//! swept, so the live waiter behind it is woken (L81). Run on the kernel's
//! own waiter list: the wake took the returning waiter, a killed one stands
//! next, and the pass-on must reach the sleeper behind it.

use super::waiters::{join, leave_early, take_live, Key};
use alloc::collections::BTreeMap;

const KEY: Key = (7, 0x1000);
const A: u32 = 10;
const D: u32 = 12;
const B: u32 = 11;

#[test]
fn a_passed_on_wake_skips_a_killed_waiter_not_yet_swept() {
    let mut queue = BTreeMap::new();
    for p in [A, D, B] {
        join(&mut queue, KEY, p);
    }
    let dead = |p: u32| p == D;
    assert_eq!(take_live(queue.get_mut(&KEY).unwrap(), 1, dead), [A], "the wake took A");
    assert_eq!(leave_early(&mut queue, KEY, A, dead), Some(B), "the pass-on spent on D");
    assert!(queue.is_empty(), "a waiter left behind: {queue:?}");
}

#[test]
fn a_pass_on_with_only_the_killed_behind_wakes_no_one() {
    let mut queue = BTreeMap::new();
    for p in [A, D] {
        join(&mut queue, KEY, p);
    }
    let dead = |p: u32| p == D;
    assert_eq!(take_live(queue.get_mut(&KEY).unwrap(), 1, dead), [A]);
    assert_eq!(leave_early(&mut queue, KEY, A, dead), None, "a wake handed to the killed");
    assert!(queue.is_empty(), "the killed waiter kept queued: {queue:?}");
}
