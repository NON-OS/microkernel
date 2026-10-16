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

//! The kernel's move to the high half, TTBR1_EL1's, once the MMU is on.
//!
//! Until now the image ran where it was loaded, one to one through TTBR0,
//! which every user address space replaces: a kernel left there has no code
//! the moment a process's root is loaded. At its high alias it is in no
//! user's tables at all. If the move cannot be made, it is said why and the
//! kernel carries on one to one, as it always has.

use super::finish::finish;
use super::info::BootInfo;
use crate::arch::aarch64::mmu::{self, KERNEL_SPACE_START};
use crate::sys::serial::{self, Line};

extern "C" {
    /// `asm/go_high.S`. Returns only if the move was refused.
    fn aarch64_go_high(info: *const (), distance: u64) -> u64;
}

/// Move, or stay and say why, then finish bring-up.
pub(super) fn move_high(info: &BootInfo) -> ! {
    if mmu::high_mapped() {
        // SAFETY: the boot CPU, alone, interrupts masked, the MMU on with the
        // high alias of the image and boot devices mapped (`high_mapped`).
        let refused = unsafe { aarch64_go_high(info as *const _ as *const (), KERNEL_SPACE_START) };
        Line::new().str(b"[MMU] relocation refused the move, code ").dec(refused).end();
    }
    serial::println(b"[MMU] the kernel stays one to one, in TTBR0");
    finish(info)
}

/// Where `aarch64_go_high` lands, at the high alias. The registers that hold
/// addresses of their own move here too: device windows, the vectors, and
/// this CPU's per-CPU block, which holds its keys.
#[no_mangle]
extern "C" fn aarch64_kernel_high(info: *const ()) -> ! {
    mmu::devices_high();
    crate::arch::aarch64::exceptions::install_vbar_el1();
    crate::arch::aarch64::cpu::percpu::install_here();
    crate::arch::aarch64::security::pac::bind_this_cpu();
    serial::println(b"[MMU] the kernel runs from TTBR1");
    // SAFETY: the caller's BootInfo, on the stack that moved with it.
    finish(unsafe { &*(info as *const BootInfo) })
}
