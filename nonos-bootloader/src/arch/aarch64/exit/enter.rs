// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The kernel entered: what it takes cleaned first, then the branch.

use super::clean::clean_handed_over;
use super::DeviceTree;
use crate::handoff::MemoryMapEntry;

/// Every region the kernel takes from the loader cleaned to the point of
/// coherency, then the kernel entered with its caches and MMU off,
/// x0 = the device tree and x1 = the handoff.
pub fn enter_kernel(entry: u64, dtb: DeviceTree, handoff: u64, map: &[MemoryMapEntry]) -> ! {
    clean_handed_over(map, dtb);
    // SAFETY: boot services have exited, `entry` is the placed image's
    // physical entry, and what the kernel reads before its own MMU is on
    // (the image, the tree, and the handoff with everything the loader
    // allocated for it) was cleaned above. Firmware tables it reads later,
    // ACPI and SMBIOS, it reads with its caches on.
    unsafe { crate::arch::aarch64::asm::nonos_arm_enter_kernel(entry, dtb.addr, handoff) }
}
