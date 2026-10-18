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

//! The boot CPU list, in device-tree order.
//!
//! Three things need a CPU's MPIDR affinity rather than a counter: PSCI wants
//! it to name the core to power on, the GIC wants it to match a redistributor
//! frame, and a secondary needs it to find out which entry of the list it is.
//! The device tree is the only place that knows, so the affinities are latched
//! here once, on the boot CPU, before any secondary is released.
//!
//! Position in this list is the kernel's dense CPU index, the boot CPU's 0:
//! contiguous by construction, which is what per-CPU tables need and what an
//! affinity does not give you.

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use super::super::stack::MAX_CPUS;
pub(in crate::arch::aarch64) use super::roster_fill::populate;

// The roster's bound is the descriptor table's: an index below one is below
// the other.
const _: () = assert!(MAX_CPUS == crate::smp::MAX_CPUS);

/// Written once by the boot CPU during `populate`, before any secondary
/// runs, and published by `COUNT`'s release store.
pub(super) static AFFINITIES: [AtomicU64; MAX_CPUS] = [const { AtomicU64::new(0) }; MAX_CPUS];
/// Each CPU's position in the device tree's list, `NO_TREE` for none.
pub(super) static TREE_AT: [AtomicU32; MAX_CPUS] = [const { AtomicU32::new(NO_TREE) }; MAX_CPUS];
pub(super) static COUNT: AtomicU32 = AtomicU32::new(0);
pub(super) const NO_TREE: u32 = u32::MAX;

/// How many CPUs the roster holds, the boot CPU included.
pub fn len() -> usize {
    COUNT.load(Ordering::Acquire) as usize
}

/// The affinity of the CPU at dense index `index`.
pub fn affinity_of(index: usize) -> Option<u64> {
    (index < len()).then(|| AFFINITIES[index].load(Ordering::Relaxed))
}

/// Where the CPU at dense index `index` sits in the device tree's list, for
/// data the tree lists in its own order.
pub fn tree_index(index: usize) -> Option<usize> {
    let at = (index < len()).then(|| TREE_AT[index].load(Ordering::Relaxed))?;
    (at != NO_TREE).then_some(at as usize)
}

/// The dense index of the CPU with `affinity`, or `None` if the roster does
/// not hold it.
pub fn index_of(affinity: u64) -> Option<usize> {
    (0..len()).find(|i| affinity_of(*i) == Some(affinity))
}
