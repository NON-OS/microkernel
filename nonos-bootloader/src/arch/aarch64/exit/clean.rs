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

//! What the loader leaves the kernel lies in its own allocations (UEFI
//! memory types LoaderCode and LoaderData) and in the device tree. Each is
//! cleaned by address: set/way maintenance does not reach a system cache
//! past the CPU's, and the kernel first reads these with its caches off.

use super::handed::handed_over;
use super::DeviceTree;
use crate::arch::aarch64::caches::clean_invalidate_to_poc;
use crate::handoff::MemoryMapEntry;

const PAGE: u64 = 4096;
/// The halt code for a map or tree whose range wraps the address space.
const WRAPPED: u8 = 0xE5;

pub(super) fn clean_handed_over(map: &[MemoryMapEntry], dtb: DeviceTree) {
    for e in map.iter().filter(|e| handed_over(e.memory_type)) {
        let Some(len) = e.page_count.checked_mul(PAGE) else {
            crate::arch::cpu::halt_with_code(WRAPPED)
        };
        if !clean_invalidate_to_poc(e.physical_start, len) {
            crate::arch::cpu::halt_with_code(WRAPPED);
        }
    }
    if !clean_invalidate_to_poc(dtb.addr, dtb.len) {
        crate::arch::cpu::halt_with_code(WRAPPED);
    }
}
