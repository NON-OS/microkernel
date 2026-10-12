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

mod acpi;
mod announce;
mod describe;
mod devices;
mod entropy;
mod gic;
mod memory;
mod platform;
mod processors;
mod psci;
mod reserved;

use crate::arch::aarch64::boot::info::BootInfo;
use crate::arch::fdt::Fdt;

use describe::describe;

/// What became of the pointer the loader left in x0.
pub enum Outcome {
    /// x0 was zero: QEMU's ELF boot, or a loader that passes no tree.
    NoTree,
    Parsed,
    /// A tree was handed over and cannot be trusted. Booting on the virt
    /// defaults instead would drive a real board at QEMU's addresses.
    Broken(&'static [u8]),
}

pub fn populate(dtb_ptr: u64, info: &mut BootInfo) -> Outcome {
    if dtb_ptr == 0 {
        return Outcome::NoTree;
    }
    // SAFETY: the MMU is still off, so x0 is the physical address the loader
    // left the blob at, and zero was turned away above. A pointer to nothing
    // takes an external abort, which the vectors already installed report.
    let fdt = match unsafe { Fdt::from_ptr(dtb_ptr as *const u8) } {
        Ok(f) => f,
        Err(e) => return Outcome::Broken(describe(e)),
    };
    info.dtb_base = dtb_ptr;
    info.dtb_size = fdt.header.totalsize as u64;
    announce::model(&fdt);
    let steps = acpi::populate(&fdt)
        .and_then(|()| memory::populate(&fdt, info))
        .and_then(|()| devices::populate(&fdt, info))
        .and_then(|()| platform::populate(&fdt))
        .and_then(|()| psci::populate(&fdt))
        .and_then(|()| processors::populate(&fdt, info))
        .and_then(|()| reserved::populate(&fdt, info));
    let seed = entropy::keep(&fdt, dtb_ptr);
    drop(fdt);
    if let Some(at) = seed {
        // SAFETY: MMU off, the parser dropped above, `at` from this blob.
        unsafe { entropy::erase(dtb_ptr, at) };
    }
    match steps {
        Ok(()) => Outcome::Parsed,
        Err(reason) => Outcome::Broken(reason),
    }
}
