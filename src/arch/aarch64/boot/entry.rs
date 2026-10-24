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

//! Where the firmware lands.
//!
//! Boot loaders on this arch pass the device tree in x0, so that is the only
//! argument. Everything the kernel needs to know about the board comes out of
//! it: RAM, console, interrupt controller, timer and CPU count.
//!
//! From `microkernel_init` onward the path is the same code x86_64 runs. The
//! two arches differ in how they answer the questions in `KernelHandoff`, not
//! in what happens afterwards.

use super::dtb_adapter::Outcome;
use super::info::BootInfo;
use crate::sys::serial;

#[no_mangle]
pub extern "C" fn kernel_entry(dtb_ptr: u64) -> ! {
    // start.S set CPACR_EL1.FPEN and VBAR_EL1; this finishes the per-CPU
    // setup: the SCTLR bits, SVE where implemented, the instruction cache.
    crate::arch::aarch64::cpu::init_cpu();

    let mut info = BootInfo::default();
    // Console next, on virt's PL011, so what follows can report its failure;
    // `init` moves it to the tree's UART. A wrong guess beats a silent boot.
    if let Err(reason) = crate::arch::aarch64::uart::init_uart(&info.uart) {
        super::refuse(b"console", reason.as_bytes());
    }
    serial::print(b"[NONOS] dtb at ");
    serial::print_hex(dtb_ptr);
    serial::println(b"");

    #[cfg(feature = "nonos-trap-proof-sp0")]
    super::trap_proof::sp_el0_vector();

    let outcome = super::dtb_adapter::populate(dtb_ptr, &mut info);
    if let Outcome::Broken(reason) = outcome {
        super::refuse(b"device tree", reason);
    }
    // Signing starts here: after the tree, whose rng-seed may key it, in a
    // function that never returns, and before `init` starts the other CPUs.
    if let Some(key) = crate::arch::aarch64::security::pac::prepare() {
        crate::arch::aarch64::security::pac::install_kernel_key!(key);
    }

    // The console, the vectors, the MMU and caches; then the move to TTBR1
    // and, from wherever the kernel then runs, the GIC, timer and the rest.
    super::init(&info);
    super::finish::set_parsed(matches!(outcome, Outcome::Parsed));
    super::high::move_high(&info)
}
