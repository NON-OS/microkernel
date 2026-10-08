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

//! Extended config space (0x100 to 0xFFF) on a PC, read-only. The port pair
//! reaches 256 bytes, so above them the MCFG window is read a page at a time,
//! as Linux raw_pci_read sends type 1 below 256 and MMCONFIG above. No x86
//! path read the window before, so it is trusted only once the first function
//! reads the same id through it as through the ports: a bad MCFG then costs
//! the extended capabilities, never a wrong value.

use core::sync::atomic::{AtomicU8, Ordering};

use crate::arch::x86_64::acpi::data::PcieSegment;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::mmio::{map_device_memory, unmap_mmio};

const UNTESTED: u8 = 0;
const TRUSTED: u8 = 1;
const REFUSED: u8 = 2;
static TRUST: AtomicU8 = AtomicU8::new(UNTESTED);

/// Map one ECAM bus window uncached (the MCFG config space for a PCIe bus). The
/// device mapping lives here, in the PCI config driver, so the broker gate sees
/// config-space mappings in the driver tree rather than in the ACPI code that
/// locates the segment. `None` when the window cannot be mapped.
pub(crate) fn map_ecam_bus(phys: u64, len: usize) -> Option<VirtAddr> {
    map_device_memory(PhysAddr::new(phys), len).ok()
}

/// All-ones, as for an absent function, when the window is missing, refused,
/// or does not cover this function.
pub fn read_extended32(bus: u8, device: u8, function: u8, offset: u16) -> u32 {
    if !(0x100..0x1000).contains(&offset) {
        return !0;
    }
    let Some(seg) = segment0() else { return !0 };
    if !trusted(&seg) {
        return !0;
    }
    ecam_read32(&seg, bus, device, function, offset & !3).unwrap_or(!0)
}

fn segment0() -> Option<PcieSegment> {
    crate::arch::x86_64::acpi::parser::pcie_segments().into_iter().find(|s| s.segment == 0)
}

fn ecam_read32(seg: &PcieSegment, bus: u8, dev: u8, func: u8, offset: u16) -> Option<u32> {
    let phys = seg.config_address(bus, dev, func, offset)?;
    let page = map_device_memory(PhysAddr::new(phys & !0xFFF), 0x1000).ok()?;
    // SAFETY: the page was just mapped uncached as device memory and the
    // offset is dword aligned inside it; ECAM answers any aligned read.
    let value = unsafe { core::ptr::read_volatile((page.as_u64() + (phys & 0xFFF)) as *const u32) };
    let _ = unmap_mmio(page);
    Some(value)
}

fn trusted(seg: &PcieSegment) -> bool {
    match TRUST.load(Ordering::Acquire) {
        TRUSTED => return true,
        REFUSED => return false,
        _ => {}
    }
    let ports = super::access::read32_unchecked(seg.start_bus, 0, 0, 0);
    let ok = ports != !0 && ecam_read32(seg, seg.start_bus, 0, 0, 0) == Some(ports);
    TRUST.store(if ok { TRUSTED } else { REFUSED }, Ordering::Release);
    let verdict = if ok { "trusted" } else { "refused: id differs from the ports" };
    crate::log::info!("[PCI] MCFG window {:#x}: {}", seg.base_address, verdict);
    ok
}
