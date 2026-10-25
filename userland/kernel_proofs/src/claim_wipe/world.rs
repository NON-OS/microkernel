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

//! The seeded world of plan section 6 (L78): one PCI function with its claims
//! entry and its wipe record, three processes (A; A2 of A's driver service; B
//! of another), their threads, two CPUs' ticks and the surface registry, all
//! drawn from one xorshift, and the ground truth the invariants judge by.

use super::kept_rule::Run;
use super::world_cases::SEEN;
use super::world_dev::Dev;
use super::world_knob::Knob;
use super::world_runs::RunM;
use super::world_thread::{Proc, Thread};
use super::world_types::{Entry, Truth, Wipe};

#[derive(Default)]
pub(super) struct World {
    pub now: u64,
    pub rng: u64,
    pub knob: Knob,
    pub dev: Dev,
    /// table.rs `WIPES` for the one function, and `PENDING`.
    pub rec: Option<Wipe>,
    pub pending: usize,
    /// The thread or tick holding `WIPES` from one move to the next.
    pub wipes_by: Option<usize>,
    /// state.rs `CLAIMS` for the one function, and its next epoch.
    pub entry: Option<Entry>,
    pub epochs: u64,
    pub runs: Vec<RunM>,
    /// kept.rs `KEPT`: (run, epoch, its state).
    pub kept: Vec<(usize, u64, Run, bool)>,
    pub procs: [Proc; 3],
    /// Process p's main thread at 3p, a sibling's release at 3p + 1, its
    /// exit at 3p + 2; the two CPUs' ticks at 9 and 10.
    pub threads: [Thread; 11],
    pub truth: Truth,
    /// The cases reached, for the floors.
    pub seen: [u32; SEEN],
    /// Bit i: invariant Ii broken; `why`, the first broken.
    pub broke: u16,
    pub why: &'static str,
}

impl World {
    /// The seeded xorshift every draw comes from.
    pub(super) fn draw(&mut self, n: u64) -> u64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        self.rng % n
    }

    pub(super) fn chance(&mut self, one_in: u64) -> bool {
        self.draw(one_in) == 0
    }
}
