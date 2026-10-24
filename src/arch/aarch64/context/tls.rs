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

//! TPIDR_EL0, the thread register EL0 reads and writes without a trap (Arm
//! ARM, TPIDR_EL0; recalled, unverified). Nothing on the exception path
//! saves or loads it, so it is the running task's until a switch: the
//! switch keeps the outgoing task's value and installs the incoming one's,
//! or one task would read, and run on, another's.

use core::arch::asm;
use core::sync::atomic::Ordering;

use crate::process::core::{CURRENT_PID, PROCESS_TABLE};

pub fn write_user_tls(base: u64) {
    // SAFETY: TPIDR_EL0 only holds a value for EL0; the kernel never
    // addresses through it.
    unsafe { asm!("msr tpidr_el0, {}", in(reg) base, options(nomem, nostack, preserves_flags)) };
}

fn read_user_tls() -> u64 {
    let base: u64;
    // SAFETY: a plain read of the EL0 thread register.
    unsafe { asm!("mrs {}, tpidr_el0", out(reg) base, options(nomem, nostack, preserves_flags)) };
    base
}

/// Keep what the current task left in TPIDR_EL0, which it may have written
/// itself since the last switch, before another task's value replaces it.
pub(crate) fn keep_outgoing_tls() {
    let pid = CURRENT_PID.load(Ordering::Acquire);
    if pid == 0 {
        return;
    }
    if let Some(pcb) = PROCESS_TABLE.find_by_pid(pid) {
        pcb.set_tls_base(read_user_tls());
    }
}
