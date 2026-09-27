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

//! A family's own pid numbers.
//!
//! Kernel pids are global. A guest reading them learns how many processes the
//! machine has started, and can watch a sibling's forks move the counter, so
//! it sees these instead: the personality is 1, the program it started is 2,
//! and each process or thread after takes the next number. None is reused
//! while the family lives, so a stale number never reaches a newer process.

use alloc::vec::Vec;

pub struct PidNs {
    map: Vec<(u32, u32)>,
    next: u32,
}

impl PidNs {
    pub fn new(personality: u32, first: u32) -> Self {
        PidNs { map: alloc::vec![(personality, 1), (first, 2)], next: 3 }
    }

    /// The number a guest sees for kernel pid `k`, given on first sight.
    /// Zero once the space is spent, which no caller reads as a process.
    pub fn outward(&mut self, k: u32) -> u32 {
        if let Some(&(_, g)) = self.map.iter().find(|(kp, _)| *kp == k) {
            return g;
        }
        let Some(after) = self.next.checked_add(1) else {
            return 0;
        };
        let g = self.next;
        self.next = after;
        self.map.push((k, g));
        g
    }

    /// The kernel pid behind a guest's number, if it names one of this family.
    pub fn inward(&self, g: u32) -> Option<u32> {
        self.map.iter().find(|(_, gp)| *gp == g).map(|(k, _)| *k)
    }
}
