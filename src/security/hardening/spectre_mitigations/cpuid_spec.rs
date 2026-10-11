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
//! IBRS, IBPB, STIBP and SSBD as each vendor reports them: Intel in CPUID
//! leaf 7 EDX, AMD in leaf 0x8000_0008 EBX. Read from leaf 7 alone, an AMD
//! part reported none of them, so SPEC_CTRL stayed unset on every CPU and no
//! switch between processes issued IBPB.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct SpecControls {
    pub(super) ibrs: bool,
    pub(super) ibpb: bool,
    pub(super) stibp: bool,
    pub(super) ssbd: bool,
}

const fn bit(word: u32, n: u32) -> bool {
    word & (1 << n) != 0
}

/// Pure: leaf 7 EDX (Intel: 26 IBRS and IBPB, 27 STIBP, 31 SSBD) and leaf
/// 0x8000_0008 EBX (AMD: 12 IBPB, 14 IBRS, 15 STIBP, 24 SSBD).
pub(super) const fn decode(leaf7_edx: u32, ext8_ebx: u32) -> SpecControls {
    SpecControls {
        ibrs: bit(leaf7_edx, 26) || bit(ext8_ebx, 14),
        ibpb: bit(leaf7_edx, 26) || bit(ext8_ebx, 12),
        stibp: bit(leaf7_edx, 27) || bit(ext8_ebx, 15),
        ssbd: bit(leaf7_edx, 31) || bit(ext8_ebx, 24),
    }
}

/// This CPU's controls; a leaf the CPU does not have reads as no bits.
#[cfg(target_arch = "x86_64")]
pub(super) fn read() -> SpecControls {
    use core::arch::x86_64::{__cpuid, __cpuid_count};
    let leaf7 = if __cpuid(0).eax >= 7 { __cpuid_count(7, 0).edx } else { 0 };
    let ext8 = if __cpuid(0x8000_0000).eax >= 0x8000_0008 { __cpuid(0x8000_0008).ebx } else { 0 };
    decode(leaf7, ext8)
}
