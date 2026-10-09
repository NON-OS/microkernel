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

//! One waiter and one waker, stepped in a chosen order. The steps are the
//! kernel's: the waiter reads its wake count, joins the list, reads the word
//! and decides, and the sleep is refused if the count moved. The waker writes
//! the word, then takes one waiter, bumps its count and wakes it if asleep.

use super::waiters::{join, leave, Key};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const KEY: Key = (7, 0x1000);
pub const PID: u32 = 42;

#[derive(Default)]
pub struct World {
    pub word: u32,
    pub queue: BTreeMap<Key, Vec<u32>>,
    pub wake_count: u64,
    pub token: u64,
    pub returned: bool,
    pub asleep: bool,
}

/// Waiter steps in the order the fixed wait takes them.
pub fn waiter_step(w: &mut World, step: usize, queue_first: bool) {
    match (step, queue_first) {
        (0, _) => w.token = w.wake_count,
        (1, true) | (2, false) => join(&mut w.queue, KEY, PID),
        (2, true) | (1, false) => {
            if w.word != 0 && !w.returned {
                if queue_first {
                    leave(&mut w.queue, KEY, PID);
                }
                w.returned = true;
            }
        }
        _ => {
            if !w.returned {
                w.asleep = w.wake_count == w.token;
            }
        }
    }
}

pub fn waker_step(w: &mut World, step: usize) {
    if step == 0 {
        w.word = 1;
        return;
    }
    if let Some(list) = w.queue.get_mut(&KEY) {
        if !list.is_empty() {
            list.remove(0);
            // wake_process: the count moves, and a sleeper is made Ready.
            w.wake_count += 1;
            w.asleep = false;
        }
    }
}
