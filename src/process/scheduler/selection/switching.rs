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

// Scheduler's PCB->user delegator. Per-arch logic lives in
// `arch::<arch>::context::switch`; this file is the call site so the
// scheduler core stays arch-neutral.

use core::sync::atomic::{AtomicBool, Ordering};

use crate::smp::MAX_CPUS;

/// Switch this CPU to `pid`, which the caller has claimed. Does not return
/// when the switch happens. When it returns, the arch layer refused before
/// loading anything, and this CPU is still on the stack and address space it
/// was on, so the bookkeeping made for the switch is put back.
pub(crate) fn switch_to_process(pid: u32) {
    let undo = super::on_cpu_switch::enter(pid);
    let asid_before = super::thread_asid::publish(pid);
    announce(pid);
    crate::arch::context::switch_to_user_pcb(pid);
    super::thread_asid::restore(asid_before);
    super::on_cpu_switch::undo(undo);
}

static ANNOUNCED: [AtomicBool; MAX_CPUS] = [const { AtomicBool::new(false) }; MAX_CPUS];

/// Once per CPU, the first time it switches into a process: the line that
/// shows a secondary CPU is doing user work at all.
fn announce(pid: u32) {
    if !cfg!(feature = "nonos-smp") {
        return;
    }
    let cpu = crate::smp::cpu_id();
    if ANNOUNCED[cpu % MAX_CPUS].swap(true, Ordering::Relaxed) {
        return;
    }
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] cpu=").dec(cpu as u64);
    l.str(b" runs user pid ").dec(pid as u64);
    l.end();
}
