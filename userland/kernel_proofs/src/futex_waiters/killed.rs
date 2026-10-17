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

//! A waiter W1 killed around its queueing, a live W2 and one wake, every order
//! (L77e): W1 joins, then leaves at once if it ended, else sleeps, never to run
//! again if killed asleep; the kill stores the end, then sweeps W1 off every
//! queue; the wake takes one live waiter, ended ones dropped uncounted.

use super::waiters::{join, leave_early, take_live, Key};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const KEY: Key = (7, 0x1000);
pub const W1: u32 = 41;
pub const W2: u32 = 42;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Knob {
    Built,
    NoForget,
    WakeCountsEnded,
    NoCheckAfterJoin,
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct World {
    pub queue: BTreeMap<Key, Vec<u32>>,
    pub steps: [u8; 4],
    pub ended: bool,
    pub woke: Option<u32>,
}

/// Paths: 0 W1, 1 W2 (joins), 2 the kill, 3 the wake. False: no move.
pub fn step(w: &mut World, path: usize, k: Knob) -> bool {
    let s = w.steps[path];
    match (path, s) {
        (0, 0) | (1, 0) => join(&mut w.queue, KEY, [W1, W2][path]),
        (0, 1) if w.ended && k != Knob::NoCheckAfterJoin => {
            let _ = leave_early(&mut w.queue, KEY, W1, |p| p == W1);
        }
        (0, 1) => {}
        (2, 0) => w.ended = true,
        (2, 1) if k != Knob::NoForget => w.queue.retain(|_, q| {
            q.retain(|&p| p != W1);
            !q.is_empty()
        }),
        (2, 1) => {}
        (3, 0) => {
            let ended = |p: u32| p == W1 && w.ended && k != Knob::WakeCountsEnded;
            let queue = w.queue.get_mut(&KEY);
            let taken = queue.map(|q| take_live(q, 1, ended)).unwrap_or_default();
            w.woke = taken.first().copied();
            w.queue.retain(|_, q| !q.is_empty());
        }
        _ => return false,
    }
    w.steps[path] += 1;
    true
}
