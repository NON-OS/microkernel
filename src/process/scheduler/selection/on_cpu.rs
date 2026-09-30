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

//! Which process's kernel stack each CPU may still be standing on.
//!
//! A process is put back on the run queue by the CPU that is switching away
//! from it, while that CPU is still executing on the process's kernel stack:
//! it has yet to pick a successor and jump. A waker on another CPU can do the
//! same to a process that is about to sleep. On one CPU nobody can resume the
//! process in that window. On several, a second CPU could claim it and
//! restore its saved context onto the very stack the first one is using.
//!
//! So each CPU names the pid it runs (`OWNED`) and, while a switch is in
//! flight, the pid it is leaving (`LEAVING`). A claim refuses a pid that
//! another CPU names in either slot. A switch writes `LEAVING` before
//! `OWNED`, and a reader loads `OWNED` before `LEAVING`, so a reader that sees
//! the new owner also sees the pid being left. `LEAVING` is cleared once the
//! CPU is known to be off that stack: back in a restored context, or at its
//! next timer tick, which can only arrive after the switch has completed.
//!
//! The single-CPU image keeps none of this: with one CPU there is no other
//! claimant, and every entry point below is a no-op there.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::smp::MAX_CPUS;

pub(super) const TRACKED: bool = cfg!(feature = "nonos-smp");

pub(super) static OWNED: [AtomicU32; MAX_CPUS] = [const { AtomicU32::new(0) }; MAX_CPUS];
pub(super) static LEAVING: [AtomicU32; MAX_CPUS] = [const { AtomicU32::new(0) }; MAX_CPUS];

pub(super) fn this_cpu() -> usize {
    crate::smp::cpu_id() % MAX_CPUS
}

fn named_by(cpu: usize, pid: u32) -> bool {
    // OWNED first: see the module comment for why the order matters.
    OWNED[cpu].load(Ordering::SeqCst) == pid || LEAVING[cpu].load(Ordering::SeqCst) == pid
}

/// Whether a CPU other than this one may still be using `pid`'s stack.
pub(crate) fn held_elsewhere(pid: u32) -> bool {
    if !TRACKED || pid == 0 {
        return false;
    }
    let me = this_cpu();
    (0..crate::smp::cpu_count().min(MAX_CPUS)).any(|cpu| cpu != me && named_by(cpu, pid))
}

/// The CPU running `pid` or still on its stack, if any. For teardown paths
/// that must not free a stack or a page table a CPU is still using.
pub fn cpu_holding(pid: u32) -> Option<usize> {
    if !TRACKED || pid == 0 {
        return None;
    }
    (0..crate::smp::cpu_count().min(MAX_CPUS)).find(|&cpu| named_by(cpu, pid))
}
