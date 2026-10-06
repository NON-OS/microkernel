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

use super::control::{stopped, CONTROL};
use crate::arch::x86_64::acpi::parser::other::amd_iommu_bases;
use crate::memory::addr::PhysAddr;
use crate::sys::serial::{self, Line};

/// The control register sits in the first page of the unit's window.
const WINDOW: usize = 4096;

/// Stop every AMD IOMMU firmware left enabled. Returns how many it stopped.
pub fn release_from_firmware() -> usize {
    let mut stopped_count = 0;
    for base in amd_iommu_bases() {
        let Ok(va) = crate::memory::mmio::map_device_memory(PhysAddr::new(base), WINDOW) else {
            serial::println(b"[AMD-VI] unit registers not mappable; left as firmware set it");
            continue;
        };
        let reg = (va.as_u64() as usize + CONTROL) as *mut u64;
        // SAFETY: eK@nonos.systems - `reg` is the control register inside the
        // uncached mapping just made of the unit's register window.
        let control = unsafe { core::ptr::read_volatile(reg) };
        if let Some(value) = stopped(control) {
            // SAFETY: as above. Stopping the unit only widens what devices
            // reach, which is the posture this kernel states for AMD-Vi.
            unsafe { core::ptr::write_volatile(reg, value) };
            let mut line = Line::new();
            line.str(b"[AMD-VI] unit at ").hex(base);
            line.str(b" was left enabled by firmware; turned off").end();
            stopped_count += 1;
        }
    }
    stopped_count
}
