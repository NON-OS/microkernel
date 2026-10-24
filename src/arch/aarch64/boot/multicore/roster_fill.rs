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

//! Latching the roster, once, on the boot CPU.

use core::sync::atomic::Ordering;

use super::super::stack::MAX_CPUS;
use super::roster::{AFFINITIES, COUNT, NO_TREE, TREE_AT};
use super::roster_order::arrange;

/// Record the CPUs the device tree reported: the boot CPU as index 0, the
/// rest in tree order (`roster_order::arrange`). An empty or foreign list
/// leaves the boot CPU alone in the roster, never an empty one: an empty
/// roster would make every dense index a fallback.
///
/// Called on the boot CPU before any secondary is started, once.
pub(in crate::arch::aarch64) fn populate(affinities: &[u64]) {
    let mut dense = [0u64; MAX_CPUS];
    let mut tree_at = [None; MAX_CPUS];
    let own = crate::arch::aarch64::cpu::mpidr_affinity();
    let n = arrange(affinities, own, &mut dense, &mut tree_at);
    if !affinities.is_empty() && tree_at[0].is_none() {
        let mut line = crate::sys::serial::Line::new();
        line.str(b"[SMP] the device tree's CPU list lacks this CPU (affinity ").hex(own);
        line.str(b"): running it alone").end();
    }
    for (i, (affinity, at)) in dense.iter().zip(tree_at).take(n).enumerate() {
        AFFINITIES[i].store(*affinity, Ordering::Relaxed);
        let at = at.and_then(|at| u32::try_from(at).ok()).unwrap_or(NO_TREE);
        TREE_AT[i].store(at, Ordering::Relaxed);
    }
    COUNT.store(n as u32, Ordering::Release);
}
