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

//! RAM and what in it is not free, from the device tree: every `/memory`
//! range, then the blob itself, the reservation block and `/reserved-memory`.
//! On an SBI platform the firmware names its own range there, and a frame
//! handed out from it would be firmware memory.

use crate::arch::fdt::find::memory::MemoryRange;
use crate::arch::fdt::Fdt;
use crate::arch::riscv64::boot::info::{BootInfo, MemoryType};

/// As many `/reserved-memory` ranges as one pass reads.
const FIRMWARE_RANGES: usize = 8;

/// False when a range cannot be kept: an entry that wraps, more than the
/// fixed storage holds, or a malformed reservation. Then nothing about free
/// RAM is known, and the boot must not guess.
pub(super) fn populate(fdt: &Fdt, info: &mut BootInfo, ram: &[MemoryRange]) -> bool {
    info.ram_base = ram[0].base;
    info.ram_size = ram[0].size;
    for r in ram {
        info.add_memory_region(r.base, r.size, MemoryType::Available);
    }
    if info.reserved.add(info.dtb_base, info.dtb_size).is_err() {
        return false;
    }
    for entry in fdt.reservations() {
        let Ok((base, size)) = entry else { return false };
        if info.reserved.add(base, size).is_err() {
            return false;
        }
    }
    let mut firmware = [(0u64, 0u64); FIRMWARE_RANGES];
    let Ok(count) = fdt.reserved_memory(&mut firmware) else { return false };
    firmware[..count].iter().all(|&(base, size)| info.reserved.add(base, size).is_ok())
}
