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

use super::info::BootInfo;

#[no_mangle]
pub extern "C" fn kernel_entry(dtb_ptr: u64) -> ! {
    let mut info = BootInfo::default();
    let parsed = super::dtb_adapter::populate(dtb_ptr, &mut info);

    super::init(&info);
    // Without the tree nothing is known about RAM or what in it the firmware
    // keeps, so the boot stops here, on the default UART, and says so.
    if !parsed {
        crate::sys::serial::println(
            b"[NONOS] riscv64: the device tree could not be read; stopping",
        );
        crate::arch::halt_loop()
    }

    // Into the path every architecture shares, as aarch64 does: the bootstrap
    // heap first, then the neutral handoff.
    crate::memory::heap::manager::init_bootstrap();
    crate::log::init();
    let handoff = crate::boot::handoff::KernelHandoff::from_riscv64(&info);
    crate::kernel_core::microkernel_init(&handoff);
    crate::kernel_core::microkernel_main()
}
