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

//! Which CPUs may still translate through a process's page tables.
//!
//! A CPU whose process has died stays on that process's kernel stack until
//! something else is runnable, and until then `on_cpu` names the pid as its
//! own. The stack must wait for it, but the tables need not: once the CPU has
//! loaded the kernel's tables, nothing on it walks the dead ones. So such a
//! CPU records the pid in `SPACE_LEFT`, and `cpu_on_tables` leaves it out,
//! which lets the process be reaped while its CPU waits for work.
//!
//! A switch clears the record before it loads any other tables (see
//! `on_cpu_switch::enter`). The single-CPU image keeps none of this.

use core::sync::atomic::{AtomicU32, Ordering};

use super::on_cpu::{this_cpu, LEAVING, OWNED, TRACKED};
use crate::smp::MAX_CPUS;

pub(super) static SPACE_LEFT: [AtomicU32; MAX_CPUS] = [const { AtomicU32::new(0) }; MAX_CPUS];

/// Load the kernel's tables on this CPU, which has stopped running the pid it
/// owns, and record that it no longer uses that pid's tables.
pub(crate) fn leave_address_space() {
    if !TRACKED {
        return;
    }
    let me = this_cpu();
    let owned = OWNED[me].load(Ordering::SeqCst);
    if owned == 0 {
        return;
    }
    let kernel = crate::memory::paging::constants::KERNEL_ASID;
    // Recorded only once the kernel's tables are loaded, never before.
    if crate::memory::paging::manager::switch_address_space(kernel).is_ok() {
        SPACE_LEFT[me].store(owned, Ordering::SeqCst);
    }
}

/// The CPU that may still translate through `pid`'s tables, if any: one
/// running it, or leaving it, unless it has left its address space already.
/// For teardown paths that free those tables.
pub fn cpu_on_tables(pid: u32) -> Option<usize> {
    if !TRACKED || pid == 0 {
        return None;
    }
    (0..crate::smp::cpu_count().min(MAX_CPUS)).find(|&cpu| {
        // Same order as `on_cpu::named_by`: OWNED before LEAVING.
        let owned = OWNED[cpu].load(Ordering::SeqCst) == pid;
        let leaving = LEAVING[cpu].load(Ordering::SeqCst) == pid;
        leaving || (owned && SPACE_LEFT[cpu].load(Ordering::SeqCst) != pid)
    })
}
