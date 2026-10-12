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
use crate::arch::aarch64::timer::{init_timer_cpu, install_on_cpu as install_preemption_tick};
use crate::arch::cpu::idle_cpu;

use super::state::CPUS_ONLINE;

#[no_mangle]
pub extern "C" fn aarch64_ap_entry() -> ! {
    // First, in this function that never returns: every CPU signs with the
    // boot CPU's key or none does (security::pac).
    match security::pac::secondary_key() {
        Ok(Some(key)) => security::pac::install_kernel_key!(key),
        Ok(None) => {}
        Err(reason) => refuse(b"secondary pointer authentication", reason.as_bytes()),
    }
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

    CPUS_ONLINE.fetch_add(1, Ordering::AcqRel);

    loop {
        idle_cpu();
    }
}
