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

//! The boot CPU's descriptor tables, SYSCALL entry, interrupt table and the
//! bootstrap allocator: what init_core_systems sets up before it reads any
//! firmware table.

#[cfg(feature = "nonos-user-entry-proof")]
use super::syscall_msrs::print_syscall_msrs;
use crate::interrupts;
use crate::sys::{idt, serial};

pub(super) fn init_cpu_tables() {
    if crate::arch::x86_64::gdt::init().is_err() {
        crate::boot::stop("arch GDT init failed", "");
    }
    serial::println(b"[NONOS] GDT configured");
    if crate::arch::x86_64::syscall::init().is_err() {
        crate::boot::stop("arch syscall init failed", "");
    }
    serial::println(b"[NONOS] SYSCALL configured");
    #[cfg(feature = "nonos-user-entry-proof")]
    print_syscall_msrs();
    // SAFETY: the boot CPU with interrupts still off, before any other CPU
    // runs; the table it loads is a static the kernel owns for its lifetime.
    unsafe {
        idt::setup();
    }
    serial::println(b"[NONOS] Early IDT configured");
    crate::memory::heap::manager::init_bootstrap();
    serial::println(b"[NONOS] Global allocator initialized");
    interrupts::init_idt();
    serial::println(b"[NONOS] Full IDT loaded");
    crate::sys::bench::mark(b"kernel_idt_ready");
}
