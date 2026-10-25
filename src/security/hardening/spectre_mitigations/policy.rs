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

//! What the kernel keeps set on each CPU, and when it must act again. Pure,
//! so the host proofs hold it.

use super::constants::{SPEC_CTRL_IBRS, SPEC_CTRL_SSBD, SPEC_CTRL_STIBP};

/// The SPEC_CTRL bits every CPU carries, from what the part offers.
pub(super) const fn spec_ctrl_bits(ibrs: bool, stibp: bool, ssbd: bool) -> u64 {
    let mut bits = 0;
    if ibrs {
        bits |= SPEC_CTRL_IBRS;
    }
    if stibp {
        bits |= SPEC_CTRL_STIBP;
    }
    if ssbd {
        bits |= SPEC_CTRL_SSBD;
    }
    bits
}

/// Whether IBRS has to be written again on every entry to the kernel. Legacy
/// IBRS only guards against predictions made before the write that set it,
/// so each entry from user mode writes it again. Enhanced IBRS
/// (ARCH_CAPABILITIES.IBRS_ALL) set once guards every later entry, and the
/// per-entry write is two MSR accesses on every system call for nothing.
pub(super) const fn rewrite_on_entry(ibrs: bool, enhanced: bool) -> bool {
    ibrs && !enhanced
}

/// Whether switching a CPU from thread group `prev` to `next` needs an
/// indirect branch prediction barrier: whenever the process changes, so one
/// process cannot steer another's indirect branches through the predictor.
/// Threads of one process share an address space and need none. `prev` is 0
/// before the CPU has run any process, which also takes the barrier.
pub(super) const fn barrier_on_switch(has_ibpb: bool, prev: u32, next: u32) -> bool {
    has_ibpb && prev != next
}
