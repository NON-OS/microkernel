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

use super::interleave::{waiter_step, waker_step, World, KEY, PID};
use super::waiters::{join, leave};
use alloc::collections::BTreeMap;

/// Every placement of the waker's two steps among the waiter's four. Returns
/// how many interleavings end with the waiter asleep after the word changed.
fn lost_wakes(queue_first: bool) -> usize {
    let mut lost = 0;
    for a in 0..6 {
        for b in (a + 1)..6 {
            let mut w = World::default();
            let (mut wi, mut ki) = (0, 0);
            for slot in 0..6 {
                if slot == a || slot == b {
                    waker_step(&mut w, ki);
                    ki += 1;
                } else {
                    waiter_step(&mut w, wi, queue_first);
                    wi += 1;
                }
            }
            if w.asleep {
                lost += 1;
            }
        }
    }
    lost
}

#[test]
fn queued_before_the_read_no_interleaving_sleeps_through_the_change() {
    assert_eq!(lost_wakes(true), 0);
}

#[test]
fn read_before_queueing_sleeps_through_a_change_the_waker_made_in_between() {
    assert!(lost_wakes(false) > 0);
}

#[test]
fn leave_drops_only_the_caller_and_an_emptied_list() {
    let mut q = BTreeMap::new();
    join(&mut q, KEY, PID);
    join(&mut q, KEY, PID + 1);
    leave(&mut q, KEY, PID);
    assert_eq!(q.get(&KEY).map(|v| v.as_slice()), Some(&[PID + 1][..]));
    leave(&mut q, KEY, PID + 1);
    assert!(q.is_empty());
    leave(&mut q, KEY, PID);
    assert!(q.is_empty());
}
