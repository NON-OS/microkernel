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

//! Whether this CPU is QEMU's software emulation (TCG).
//!
//! TCG emulates SSE4 and AVX2 code far more slowly than plain x86-64 code,
//! so under it the plain build answers first. QEMU names itself in the
//! hypervisor vendor leaf, CPUID 0x4000_0000, as "TCGTCGTCGTCG", and sets
//! the hypervisor bit in leaf 1 for it to be read at all.

use core::arch::x86_64::__cpuid;

/// True when CPUID says this is QEMU's TCG. Leaf 0x4000_0000 is read only
/// when leaf 1 sets the hypervisor bit that promises it.
pub fn under_tcg() -> bool {
    if __cpuid(1).ecx & (1 << 31) == 0 {
        return false;
    }
    let leaf = __cpuid(0x4000_0000);
    let mut vendor = [0u8; 12];
    vendor[..4].copy_from_slice(&leaf.ebx.to_le_bytes());
    vendor[4..8].copy_from_slice(&leaf.ecx.to_le_bytes());
    vendor[8..].copy_from_slice(&leaf.edx.to_le_bytes());
    &vendor == b"TCGTCGTCGTCG"
}
