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

//! The register operand of a TLB invalidate by address, `TLBI VAAE1IS`
//! (Arm ARM, DDI 0487, the TLBI VAAE1IS operand). With no ASID field, bits
//! 63:48 are RES0, bits 47:44 are TTL and bits 43:0 carry VA[55:12].
//!
//! TTL is a hint, defined on FEAT_TTL (Armv8.4): a non-zero value names the
//! granule and the level of the entry being removed, and the architecture
//! need not invalidate an entry the hint does not describe. A kernel address
//! shifted right by 12 carries ones up to bit 51, so bits 47:44 read 0b1111,
//! a 64K granule at level 3, while NONOS maps with the 4K granule. The VA
//! field is therefore masked, and TTL is left at 0, "no hint", so one operand
//! is right for an entry at any level: a 4K page or a 2M or 1G block.

/// Bits 43:0 of the operand, which hold VA[55:12].
pub const VA_FIELD: u64 = (1 << 44) - 1;

/// The operand for `TLBI VAAE1IS` that invalidates `va`, any ASID, any level.
#[inline(always)]
pub const fn vaae1is(va: u64) -> u64 {
    (va >> 12) & VA_FIELD
}
