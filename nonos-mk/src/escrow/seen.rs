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

//! The agent answers each Equi-X solution once (plan/22 step 1: it
//! rate-limits per solution). The newest SEEN_MAX are remembered; past that
//! the oldest is forgotten, and a replay of it costs one more answer.

use std::collections::{HashSet, VecDeque};

pub const SEEN_MAX: usize = 1 << 16;

#[derive(Default)]
pub struct Seen {
    set: HashSet<[u8; 48]>,
    order: VecDeque<[u8; 48]>,
}

impl Seen {
    /// True the first time this (solution, buyer_pk) is offered.
    pub fn take(&mut self, solution: &[u8; 16], buyer_pk: &[u8; 32]) -> bool {
        let mut k = [0u8; 48];
        k[..16].copy_from_slice(solution);
        k[16..].copy_from_slice(buyer_pk);
        if !self.set.insert(k) {
            return false;
        }
        self.order.push_back(k);
        if self.order.len() > SEEN_MAX {
            self.order.pop_front().map(|old| self.set.remove(&old));
        }
        true
    }
}
