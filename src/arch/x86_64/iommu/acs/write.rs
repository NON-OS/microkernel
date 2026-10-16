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

//! One write to extended configuration space, for the kernel's own use: a
//! switch port's ACS controls at boot (L72). Made only where the dword it
//! lands in reads back through the trusted MCFG window first, so a missing
//! or refused window, or a function that is not there, writes nothing.

use crate::memory::addr::PhysAddr;
use crate::memory::mmio::{map_device_memory, unmap_mmio};

/// Writes the 16 bits at `offset`, 100h to FFEh and even; false when nothing
/// was written.
pub(super) fn write_extended16(bus: u8, device: u8, function: u8, offset: u16, value: u16) -> bool {
    if !(0x100..0x1000).contains(&offset) || offset & 1 != 0 {
        return false;
    }
    if crate::drivers::pci::config::read_extended32(bus, device, function, offset & !3) == !0 {
        return false;
    }
    let segments = crate::arch::x86_64::acpi::parser::pcie_segments();
    let Some(seg) = segments.into_iter().find(|s| s.segment == 0) else { return false };
    let Some(phys) = seg.config_address(bus, device, function, offset) else { return false };
    let Ok(page) = map_device_memory(PhysAddr::new(phys & !0xFFF), 0x1000) else { return false };
    // SAFETY: eK@nonos.systems - the page was just mapped uncached as device
    // memory, the offset is even and inside it, and ECAM takes an aligned
    // 16-bit write; the window answered for this function a moment ago.
    unsafe { core::ptr::write_volatile((page.as_u64() + (phys & 0xFFF)) as *mut u16, value) };
    let _ = unmap_mmio(page);
    true
}
