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

//! What happens when the heap is exhausted.
//!
//! Nothing here may allocate. The console sink and its decimal printer both
//! work without the allocator, which is why the report goes through them
//! rather than the formatter. The heap holds the session's secrets, so an
//! exhausted one ends as a panic does: wiped, then off.

use core::alloc::Layout;

use crate::sys::serial;

pub fn handle_oom(layout: Layout) -> ! {
    serial::println(b"");
    serial::println(b"[OOM] ALLOCATION FAILED");
    serial::print(b"[OOM] Requested size: ");
    serial::print_dec(layout.size() as u64);
    serial::print(b" bytes, align: ");
    serial::print_dec(layout.align() as u64);
    serial::println(b"");

    crate::syscall::microkernel::memory::dump_mmap_accounting();
    crate::kernel_core::surface_registry::dump_surface_accounting();
    serial::println(b"[OOM] to the panic path: wipe, power off");

    // No VGA text: 0xB8000 is in no process's address space, so the write
    // faulted when the heap ran out in one, and no UEFI machine shows it.
    crate::boot::panic::fatal()
}
