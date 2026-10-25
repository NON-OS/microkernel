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

//! Which root a walk of the tables must start from for a given address.
//!
//! x86_64 and riscv64 translate both halves through one root. aarch64 splits
//! them: TTBR1_EL1 translates every address whose top bits are set, TTBR0_EL1
//! the rest (Arm ARM, DDI 0487, the translation regime's two ranges). A walk
//! of a kernel address from TTBR0's root reads a table the CPU never uses for
//! it, and a mapping made there is one the kernel cannot reach.

/// The root this CPU translates `va` through.
#[inline]
pub fn root_for(va: u64) -> u64 {
    root_in(super::read_root(), va)
}

/// The root `va` is translated through in the address space whose own root
/// is `space_root`: that root, unless the address is the kernel's own half.
#[inline]
pub fn root_in(space_root: u64, va: u64) -> u64 {
    #[cfg(target_arch = "aarch64")]
    if va >> 63 == 1 {
        return crate::arch::aarch64::mmu::read_ttbr1() & 0x0000_FFFF_FFFF_F000;
    }
    let _ = va;
    space_root
}
