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

//! A joined first and has not read the word; B joined behind it and sleeps.
//! The waker writes the word, then takes and wakes one waiter. Every place of
//! A's read among those two steps is run.

use super::waiters::{join, leave, leave_early, Key};
use alloc::{collections::BTreeMap, vec::Vec};

const KEY: Key = (7, 0x1000);
const A: u32 = 10;
const B: u32 = 11;

/// Whether A returned on the change and B ends asleep with no wake.
fn b_stranded(at: usize, pass_on: bool) -> bool {
    let mut queue: BTreeMap<Key, Vec<u32>> = BTreeMap::new();
    join(&mut queue, KEY, A);
    join(&mut queue, KEY, B);
    let (mut word, mut b_asleep, mut a_returned) = (0u32, true, false);
    for step in 0..3 {
        if step == at {
            if word != 0 {
                a_returned = true;
                if !pass_on {
                    leave(&mut queue, KEY, A);
                } else if let Some(next) = leave_early(&mut queue, KEY, A) {
                    b_asleep &= next != B;
                }
            }
            continue;
        }
        if word == 0 {
            word = 1;
        } else if let Some(list) = queue.get_mut(&KEY).filter(|l| !l.is_empty()) {
            b_asleep &= list.remove(0) != B;
        }
    }
    a_returned && b_asleep
}

#[test]
fn a_wake_spent_on_a_returning_waiter_strands_the_one_behind_it() {
    assert!((0..3).any(|at| b_stranded(at, false)));
}

#[test]
fn passed_on_the_wake_reaches_the_waiter_behind_in_every_placement() {
    assert!((0..3).all(|at| !b_stranded(at, true)));
}

#[test]
fn leave_early_takes_no_one_when_the_caller_was_still_listed() {
    let mut queue = BTreeMap::new();
    join(&mut queue, KEY, A);
    join(&mut queue, KEY, B);
    assert_eq!(leave_early(&mut queue, KEY, A), None);
    assert_eq!(queue.get(&KEY).map(|l| l.as_slice()), Some(&[B][..]));
    queue.insert(KEY, Vec::new());
    assert_eq!(leave_early(&mut queue, KEY, A), None);
    assert!(queue.is_empty());
}
