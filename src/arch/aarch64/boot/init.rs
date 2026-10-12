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
use crate::arch::aarch64::{cpu, exceptions, gic, mmu, security, uart};

pub fn init(boot_info: &BootInfo) {
    if let Err(reason) = uart::init_uart(&boot_info.uart) {
        super::refuse(b"console", reason.as_bytes());
    }
    cpu::init_cpu();
    // Latch the CPU list before anything asks which CPU it is running on. The
    // GIC does, one call below, and its answer decides which redistributor
    // this core talks to.
    //
    // SAFETY: this is the boot CPU, this runs once, and no secondary has been
    // released yet, so nothing else can be touching the roster.
    unsafe {
        super::multicore::roster::populate(
            &boot_info.cpu_affinities[..boot_info.cpu_affinity_count],
        );
    }
    super::platform_windows::publish(boot_info);
    exceptions::install_vbar_el1();
    security::init_all();
    mmu::init_mmu(boot_info);
    let gic = gic::init_gic(
        boot_info.gic_version,
        boot_info.gic_dist_base,
        boot_info.gic_cpu_base,
        boot_info.gic_redist_base,
    );
    if let Err(reason) = gic {
        super::refuse(b"gic", reason.as_bytes());
    }
    super::start_clock::start_clock(boot_info);
    if super::multicore::roster::len() > 1 {
        super::multicore::start_secondary_cpus(boot_info);
    }

    // Firmware hands the kernel a CPU with DAIF masked and `_start` keeps it
    // that way, so nothing is delivered until this point. It goes last, once the
    // vectors are installed and the GIC and timer can say who is asking:
    // unmasking earlier means the first interrupt arrives before anything can
    // handle it.
    //
    // x86_64 does the same thing with `sti` inside its own early setup, which
    // this path does not share. Without it the timer never ticks, the clock
    // never advances, and anything waiting on elapsed time waits forever with
    // the machine idle and no sign of what is wrong.
    cpu::enable_interrupts();
}
