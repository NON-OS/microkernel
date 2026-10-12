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

//! Events waiting for the holder of the device they belong to, at most
//! `DEPTH` for each, the oldest dropped first and counted when a holder
//! does not drain them.

use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;

const DEPTH: usize = 64;

#[derive(Default)]
pub struct Queue {
    waiting: BTreeMap<u64, VecDeque<(u16, u16)>>,
    pub dropped: u64,
}

impl Queue {
    /// One event on `pin` of controller `uid`, for device `key`.
    pub fn push(&mut self, key: u64, uid: u16, pin: u16) {
        let q = self.waiting.entry(key).or_default();
        if q.len() == DEPTH {
            q.pop_front();
            self.dropped += 1;
        }
        q.push_back((uid, pin));
    }

    /// Up to `max` events for `key`, oldest first.
    pub fn drain(&mut self, key: u64, max: usize) -> Vec<(u16, u16)> {
        let Some(q) = self.waiting.get_mut(&key) else { return Vec::new() };
        let n = q.len().min(max);
        q.drain(..n).collect()
    }
}
