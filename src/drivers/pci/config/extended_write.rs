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

//! One dword of extended config space written through the MCFG window
//! (ACS control). Only where read_extended32 already reads the same
//! dword, so the window is trusted and covers the function.

use super::extended::read_extended32;
use crate::memory::addr::PhysAddr;
use crate::memory::mmio::{map_device_memory, unmap_mmio};

/// False when the window does not reach this dword.
pub fn write_extended32(bus: u8, device: u8, function: u8, offset: u16, value: u32) -> bool {
    let offset = offset & !3;
    if !(0x100..0x1000).contains(&offset) || read_extended32(bus, device, function, offset) == !0 {
        return false;
    }
    let segments = crate::arch::x86_64::acpi::parser::pcie_segments();
    let Some(seg) = segments.into_iter().find(|s| s.segment == 0) else {
        return false;
    };
    let Some(phys) = seg.config_address(bus, device, function, offset) else {
        return false;
    };
    let Ok(page) = map_device_memory(PhysAddr::new(phys & !0xFFF), 0x1000) else {
        return false;
    };
    // SAFETY: the page was just mapped uncached as device memory and the
    // offset is dword aligned inside it; ECAM takes any aligned write.
    unsafe { core::ptr::write_volatile((page.as_u64() + (phys & 0xFFF)) as *mut u32, value) };
    let _ = unmap_mmio(page);
    true
}
