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

//! What NONOS tells each processor's _OSC it implements: every capability
//! bit tied to the kernel feature that honours it, and a bit is asked only
//! while that feature is in the kernel. The bits are the processor
//! vendor's (Intel's processor vendor-specific ACPI interface; recalled,
//! unverified), not ACPI's. The kernel enters no _CST or _LPI state, sets
//! no P-state and throttles nothing, so every feature is false and the
//! word is 0: firmware hears only that an operating system that knows
//! processor _OSC is present. A feature turns true only in the unit that
//! lands A's code for it, never here. Pure: `capsule_acpi_proofs` mounts it.

pub use super::caps_kernel::{Kernel, KERNEL};

pub struct Row {
    pub bit: u32,
    pub what: &'static str,
    pub honoured: fn(&Kernel) -> bool,
}

pub const ROWS: [Row; 9] = [
    Row { bit: 0, what: "P-states by MSR", honoured: |k| k.pstates },
    Row { bit: 1, what: "C1 I/O then halt", honoured: |k| k.cstates_io },
    Row { bit: 2, what: "T-states by MSR", honoured: |k| k.tstates },
    Row { bit: 3, what: "C-states per processor", honoured: |k| k.coordination && k.cstates_io },
    Row { bit: 4, what: "P/T-states per processor", honoured: |k| k.coordination && k.pstates },
    Row { bit: 5, what: "C1 native", honoured: |k| k.cstates_native },
    Row { bit: 8, what: "C2/C3 native", honoured: |k| k.cstates_native },
    Row { bit: 9, what: "_CSD coordination", honoured: |k| k.coordination && k.cstates_native },
    Row { bit: 12, what: "CPPC", honoured: |k| k.cppc },
];

/// The word for a kernel with `k`'s features: each row's bit only when
/// its feature is there.
pub fn word_for(k: &Kernel) -> u32 {
    ROWS.iter().filter(|r| (r.honoured)(k)).fold(0, |w, r| w | 1 << r.bit)
}

pub fn word() -> u32 {
    word_for(&KERNEL)
}
