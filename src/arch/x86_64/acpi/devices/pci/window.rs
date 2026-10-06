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

//! One bus's slice of an MCFG ECAM window, mapped as device memory for as
//! long as the walk of that bus takes.
//!
//! The MCFG gives physical addresses. Dereferencing them as pointers only
//! worked where low memory happened to be identity mapped, and ECAM sits
//! high (0xE0000000 on Gemini Lake, above 4 GiB on many newer parts), so
//! each bus's 1 MiB is mapped uncached here and unmapped when the walk moves
//! on.

use crate::arch::x86_64::acpi::data::PcieSegment;
use crate::memory::addr::{PhysAddr, VirtAddr};

const BUS_BYTES: u64 = 1 << 20;

pub(super) struct BusWindow {
    va: u64,
}

impl BusWindow {
    pub(super) fn map(seg: &PcieSegment, bus: u8) -> Option<Self> {
        let phys = seg.config_address(bus, 0, 0, 0)?;
        let va =
            crate::memory::mmio::map_device_memory(PhysAddr::new(phys), BUS_BYTES as usize).ok()?;
        Some(Self { va: va.as_u64() })
    }

    fn at(&self, device: u8, function: u8, offset: u16) -> Option<u64> {
        if device >= 32 || function >= 8 || offset >= 4096 {
            return None;
        }
        Some(self.va + ((device as u64) << 15) + ((function as u64) << 12) + offset as u64)
    }

    pub(super) fn read8(&self, device: u8, function: u8, offset: u16) -> u8 {
        match self.at(device, function, offset) {
            // SAFETY: inside this bus's mapped 1 MiB of ECAM.
            Some(a) => unsafe { core::ptr::read_volatile(a as *const u8) },
            None => 0xFF,
        }
    }

    pub(super) fn read16(&self, device: u8, function: u8, offset: u16) -> u16 {
        match self.at(device, function, offset & !1) {
            // SAFETY: inside this bus's mapped 1 MiB of ECAM, 2-byte aligned.
            Some(a) => unsafe { core::ptr::read_volatile(a as *const u16) },
            None => 0xFFFF,
        }
    }
}

impl Drop for BusWindow {
    fn drop(&mut self) {
        let _ = crate::memory::mmio::unmap_mmio(VirtAddr::new(self.va));
    }
}
