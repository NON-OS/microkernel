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

//! unit_base's check against this machine: its memory map and the CPU's
//! physical address width. Without a memory map no base can be shown
//! not to be RAM, so every base is refused, as MMIO grants are.

use super::unit_base::{check_base, BadBase};
#[cfg(feature = "nonos-arch-iommu")]
use super::unit_base::{AMD_WINDOW, DRHD_WINDOW};
use crate::hardware::broker::mmio::boot_ram_ranges;
use crate::sys::serial;

#[cfg(feature = "nonos-arch-iommu")]
const _: () = {
    use crate::arch::x86_64::iommu::{amd_vi::regs::WINDOW, unit::access::UNIT_WINDOW};
    assert!(DRHD_WINDOW == UNIT_WINDOW as u64 && AMD_WINDOW == WINDOW as u64);
};

/// Ok when `window` bytes at `base` may be mapped beside `kept`.
pub(super) fn admit(base: u64, window: u64, kept: &[u64]) -> Result<(), BadBase> {
    let Some(ram) = boot_ram_ranges() else {
        serial::println(b"[ACPI] IOMMU register base refused: no memory map");
        return Err(BadBase::OverRam);
    };
    let checked = check_base(base, window, phys_bits(), &ram, kept);
    if let Err(why) = checked {
        let line = alloc::format!("[ACPI] IOMMU register base refused: {:?}", why);
        serial::println(line.as_bytes());
    }
    checked
}

/// MAXPHYADDR from CPUID 0x80000008, or 36 when the leaf is absent.
fn phys_bits() -> u32 {
    let top = core::arch::x86_64::__cpuid(0x8000_0000).eax;
    if top < 0x8000_0008 {
        return 36;
    }
    core::arch::x86_64::__cpuid(0x8000_0008).eax & 0xFF
}
