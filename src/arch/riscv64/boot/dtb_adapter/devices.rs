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

//! What the device tree says of the boot devices: the console UART, the PLIC,
//! the harts, the timebase and the boot hart's ISA. Each is optional here; the
//! boot path falls back to QEMU virt's layout for what the tree leaves out.

use crate::arch::fdt::find::cpus;
use crate::arch::fdt::find::isa::find_boot_hart as find_isa;
use crate::arch::fdt::find::plic::find as find_plic;
use crate::arch::fdt::find::timebase::find_riscv64 as find_timebase;
use crate::arch::fdt::find::uart::{find as find_uart, UartKind};
use crate::arch::fdt::Fdt;
use crate::arch::riscv64::boot::info::BootInfo;
use crate::arch::riscv64::cpu::caps;
use crate::arch::riscv64::timer::set_frequency;

pub(super) fn populate(fdt: &Fdt, info: &mut BootInfo) {
    if let Ok(Some(u)) = find_uart(fdt) {
        if u.kind == UartKind::Ns16550 {
            info.uart_base = u.base;
        }
    }

    if let Ok(Some(p)) = find_plic(fdt) {
        info.plic_base = p.base;
    }

    let mut hart_ids = [0u64; 64];
    if let Ok(n) = cpus::find(fdt, &mut hart_ids) {
        if n > 0 {
            info.hart_count = n as u32;
        }
    }

    if let Ok(Some(freq)) = find_timebase(fdt) {
        set_frequency(freq);
    }

    if let Ok(Some(flags)) = find_isa(fdt) {
        caps::configure(flags);
    }
}
