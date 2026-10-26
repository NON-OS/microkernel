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

use core::fmt::Write;
use core::panic::PanicInfo;

struct SerialWriter;

impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::sys::serial::print(s.as_bytes());
        Ok(())
    }
}

fn serial_print(args: core::fmt::Arguments<'_>) {
    let _ = SerialWriter.write_fmt(args);
}

// Panic path: serial trace and the GOP band, then `end`: stop every other
// online CPU and wait for each to say it halted, wipe what can be wiped
// without a lock and power off (`zerostate::panic_wipe_and_off`, which
// says the order). `end` is taken before anything is printed, so a fault or
// a second panic in the printing goes straight to the wipe. The receiving
// side is the IPI vector itself (`arch::interrupt_controller::ipi_dispatch::handlers::panic`),
// which halts without returning; on a single-CPU runtime the broadcast
// targets nobody and the local halt is the whole story. No VGA text banner:
// 0xB8000 is in no process's address space, so writing it from a panic in
// one faulted, and no UEFI machine shows it.
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    super::end::end(|| {
        serial_print(format_args!("\n!!! KERNEL PANIC !!!\n"));
        serial_print(format_args!("{}\n", info));
        crate::log::dbg_ring::dbg_drain_to_serial();
        // The VGA banner is invisible on UEFI machines; paint the GOP
        // framebuffer, through the kernel's own mapping of it, so real
        // hardware shows where the kernel died.
        if let Some(loc) = info.location() {
            crate::sys::boot_log::panic_screen(loc.file(), loc.line());
        } else {
            crate::sys::boot_log::panic_screen("unknown location", 0);
        }
    })
}
