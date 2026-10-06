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
//!
//! A personality hosts one family, so the numbers are kept in one place and
//! any handler can write a guest's number into what it hands back: a siginfo
//! or a waitid answer carries a pid, not only a return value.

use super::pid_space::{inward, outward, SPACE};

/// The family's numbering. Made once, when the family is.
pub struct PidNs;

impl PidNs {
    pub fn new(personality: u32, first: u32) -> Self {
        *SPACE.0.borrow_mut() = (alloc::vec![(personality, 1), (first, 2)], 3);
        PidNs
    }

    /// The number a guest sees for kernel pid `k`, given on first sight.
    /// Zero once the space is spent, which no caller reads as a process.
    pub fn outward(&mut self, k: u32) -> u32 {
        outward(k)
    }

    /// The kernel pid behind a guest's number, if it names one of this family.
    pub fn inward(&self, g: u32) -> Option<u32> {
        inward(g)
    }
}
