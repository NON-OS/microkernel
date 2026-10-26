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

//! The registry as the L35 path sees it: slots naming the run they were
//! registered over, attach records, orphans, and the runs held for them.
//! model.rs and exit_model.rs step it as the kernel does.

pub const GPU: u32 = 7;
pub const COMPOSITOR: u32 = 3;
pub const FB: u32 = 0x11;
/// The GPU's framebuffer grant: four pages at 1 MiB, one run.
pub const START: u64 = 0x10_0000;
pub const LEN: u64 = 0x4000;
pub const RUN: (u64, u64) = (START, LEN);
/// A page no grant holds.
pub const HEAP: u64 = 0x80_0000;

pub fn grant_frames() -> Vec<u64> {
    (0..4).map(|i| START + i * 0x1000).collect()
}

/// A surface's slot; `grant` is the run register_grant.rs recorded.
pub struct Slot {
    pub handle: u32,
    pub owner: u32,
    pub frames: Vec<u64>,
    pub grant: Option<(u64, u64)>,
}

/// orphans.rs `Orphan`. Its `tables` is 0 here: no exited group's tables
/// still map the frames (L49).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Orphan {
    pub owner: u32,
    pub handle: u32,
    pub frames: Vec<u64>,
    pub free: Vec<u64>,
    pub grant: Option<(u64, u64)>,
}

#[derive(Default)]
pub struct Registry {
    pub slots: Vec<Slot>,
    /// (handle, pid)
    pub attaches: Vec<(u32, u32)>,
    pub orphans: Vec<Orphan>,
    /// grant_run.rs `RUNS`.
    pub runs: Vec<(u64, u64)>,
    /// Runs back with the broker, in order.
    pub given_back: Vec<(u64, u64)>,
    /// Frames freed one by one, as orphans_free.rs frees an orphan's list.
    pub pages_freed: Vec<u64>,
}

pub fn slot(handle: u32, owner: u32, frames: Vec<u64>, grant: Option<(u64, u64)>) -> Slot {
    Slot { handle, owner, frames, grant }
}

/// snapshot.rs `has_foreign_holder`: a record of `handle` by another pid.
pub fn foreign(attaches: &[(u32, u32)], handle: u32, owner: u32) -> bool {
    attaches.iter().any(|&(h, p)| h == handle && p != owner)
}
