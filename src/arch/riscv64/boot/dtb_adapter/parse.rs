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

use crate::arch::fdt::find::memory::{find as find_memory, MemoryRange};
use crate::arch::fdt::Fdt;
use crate::arch::riscv64::boot::info::BootInfo;

pub fn populate(dtb_ptr: u64, info: &mut BootInfo) -> bool {
    // SAFETY: paging is off, so a0 is the blob's physical address; zero is
    // refused before any read, and a bad pointer faults before stvec is set.
    let fdt = match unsafe { Fdt::from_ptr(dtb_ptr as *const u8) } {
        Ok(f) => f,
        Err(_) => return false,
    };
    info.dtb_base = dtb_ptr;
    info.dtb_size = fdt.header.totalsize as u64;

    let mut ranges = [MemoryRange { base: 0, size: 0 }; 8];
    let mem_count = match find_memory(&fdt, &mut ranges) {
        Ok(n) => n,
        Err(_) => return false,
    };
    if mem_count == 0 || !super::memory::populate(&fdt, info, &ranges[..mem_count]) {
        return false;
    }

    super::devices::populate(&fdt, info);

    // The tree's `/chosen` `rng-seed` keys the RNG; once kept, its bytes are
    // zeroed in the blob so nothing reading the tree later finds them.
    let seed = crate::arch::common::boot_seed::keep(&fdt, dtb_ptr);
    drop(fdt);
    if let Some(at) = seed {
        // SAFETY: `at` came from `keep` on this blob, the parser is dropped,
        // and no other reference into the blob is live.
        unsafe { crate::arch::common::boot_seed::erase(dtb_ptr as *mut u8, at) };
    }
    true
}
