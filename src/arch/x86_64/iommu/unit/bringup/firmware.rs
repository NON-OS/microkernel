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

//! Firmware's protected memory regions let go once every unit translates
//! with the kernel's tables (iommu::handover). One that will not turn off
//! only blocks DMA to its ranges; it is said, and bring-up stands.

use crate::arch::x86_64::iommu::unit::enable::release_protected_regions;
use crate::arch::x86_64::iommu::unit::report::units;

pub(super) fn release_protected_regions_all() {
    for unit in units() {
        // SAFETY: eK@nonos.systems - every unit is in service with the
        // kernel's tables: bring-up's loop above this call returned Ok.
        if unsafe { release_protected_regions(&unit.unit) }.is_err() {
            crate::sys::serial::println(b"[VT-D] firmware protected regions would not turn off");
        }
    }
}
