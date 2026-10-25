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

//! This CPU's per-CPU block, named by TPIDR_EL1: the block whose index is
//! this CPU's place in the roster, as the PAC keys (security::pac) and the
//! scheduler's per-CPU state expect. The boot CPU is the roster's first.
//!
//! Installed before anything reads it, and again after the kernel moves to
//! its high alias: the block is in the image, so its address moves too, and
//! the one-to-one address stops being reachable once a user root is loaded.

use super::id::mpidr_affinity;
use crate::arch::aarch64::boot::multicore::roster;

/// Point TPIDR_EL1 at this CPU's block. False when this CPU is not in the
/// roster, which then leaves the register as it was.
pub fn install_here() -> bool {
    let Some(index) = roster::index_of(mpidr_affinity()) else {
        return false;
    };
    crate::smp::percpu::init_ap(index);
    true
}
