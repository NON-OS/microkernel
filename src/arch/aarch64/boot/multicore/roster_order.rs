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

//! The device tree's CPU list laid out as the kernel's dense CPU index.
//! Pure, so the proofs run it.

/// Fill `dense` with the CPUs `tree` lists, as MPIDR affinities: the boot
/// CPU (`own`) first, as index 0, then the others in tree order, each once,
/// as many as `dense` holds. `tree_at[i]` is where `dense[i]` sits in
/// `tree`, for data the tree lists in its own order (spin-table release
/// addresses, idle-state lists). A tree that does not list the boot CPU is
/// not this machine's list, or not all of it: the roster is then the boot
/// CPU alone, with no tree position. Returns how many were written.
pub fn arrange(tree: &[u64], own: u64, dense: &mut [u64], tree_at: &mut [Option<usize>]) -> usize {
    let capacity = dense.len().min(tree_at.len());
    if capacity == 0 {
        return 0;
    }
    let boot = tree.iter().position(|affinity| *affinity == own);
    dense[0] = own;
    tree_at[0] = boot;
    if boot.is_none() {
        return 1;
    }
    let mut filled = 1;
    for (at, affinity) in tree.iter().enumerate() {
        if filled == capacity {
            break;
        }
        // Each CPU once: a second entry with the same affinity names no
        // other core, and two indices for one core would alias.
        if dense[..filled].contains(affinity) {
            continue;
        }
        dense[filled] = *affinity;
        tree_at[filled] = Some(at);
        filled += 1;
    }
    filled
}
