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

mod devices;
mod gic;
mod memory;
mod processors;

use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::fdt::Fdt;

pub fn populate(dtb_ptr: u64, info: &mut BootInfo) -> bool {
    // SAFETY: the MMU is still off, so x0 is read as the physical address the
    // loader left the blob at. A loader that passes no tree passes zero, which
    // `from_ptr` refuses before reading; one that passes a pointer to nothing
    // takes an external abort, which the vectors already installed report.
    let fdt = match unsafe { Fdt::from_ptr(dtb_ptr as *const u8) } {
        Ok(f) => f,
        Err(_) => return false,
    };
    info.dtb_base = dtb_ptr;
    info.dtb_size = fdt.header.totalsize as u64;

    if !memory::populate(&fdt, info) {
        return false;
    }
    devices::populate(&fdt, info);
    processors::populate(&fdt, info);
    true
}
