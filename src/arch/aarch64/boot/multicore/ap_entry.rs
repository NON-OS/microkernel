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

use core::sync::atomic::Ordering;

use crate::arch::aarch64::boot::refuse;
use crate::arch::aarch64::cpu;
use crate::arch::aarch64::exceptions::install_vbar_el1;
use crate::arch::aarch64::gic::init_gic_cpu;
use crate::arch::aarch64::security;
use crate::arch::aarch64::timer::{arm_on_secondary as install_preemption_tick, init_timer_cpu};
use crate::arch::cpu::idle_cpu;

use super::outside::park_outside_roster;
use super::roster;
use super::state::CPUS_ONLINE;

#[no_mangle]
pub extern "C" fn aarch64_ap_entry() -> ! {
    // A CPU the roster does not hold stops before anything below indexes a
    // per-CPU table by it.
    let Some(index) = roster::index_of(cpu::mpidr_affinity()) else {
        park_outside_roster(cpu::mpidr_affinity())
    };
    // Then, in this function that never returns: every CPU signs with the
    // boot CPU's key or none does (security::pac). The MMU is already on, from
    // `_aarch64_secondary_start`, and TCR_EL1 does not change after this, so a
    // return address signed here is authenticated where it was signed. The
    // calls above ran with EnIA clear (SCTLR_EL1 reset in start.S), so their
    // sign and authenticate were both no-ops; the boot CPU had to wait for
    // `init` to finish before installing for the same reason.
    match security::pac::secondary_key() {
        Ok(Some(key)) => security::pac::install_kernel_key!(key),
        Ok(None) => {}
        Err(reason) => refuse(b"secondary pointer authentication", reason.as_bytes()),
    }
    // The keys go into this CPU's per-CPU block, so it is named first.
    cpu::percpu::install_here();
    security::pac::bind_this_cpu();
    install_vbar_el1();
    cpu::init_cpu();
    security::init_all();
    if let Err(reason) = init_gic_cpu() {
        refuse(b"secondary gic", reason.as_bytes());
    }
    init_timer_cpu();
    if let Err(reason) = install_preemption_tick() {
        refuse(b"secondary timer tick", reason.as_bytes());
    }

    // Its own descriptor, at its roster index, before it counts as online:
    // `smp::cpu_id()` answers that index, and IPIs find it by this id.
    crate::smp::register_secondary(index, cpu::cpu_affinity());
    CPUS_ONLINE.fetch_add(1, Ordering::AcqRel);

    loop {
        idle_cpu();
    }
}
