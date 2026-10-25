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

//! A process's devices go back when its last thread ends, not when a worker
//! does: claims, MMIO, DMA, IRQ and port grants are held by the thread group
//! (L32). Teardown and finalize both ask, so whichever of two threads ending
//! together finishes last gives them back, and a second release is a no-op.

use crate::hardware::broker::{self, holder_of, others_alive};
use crate::process::core::{Pid, ProcessState, PROCESS_TABLE};

/// False only when the group's devices are still to go but a CPU still runs
/// one of its threads: finalize then tries again on a later tick (L77).
pub(super) fn release_if_last(pid: Pid, current: bool, wait: bool) -> bool {
    // The ACPI capsule's events disarmed before its SCI grant goes, at the
    // exit of the pid that serves it and before any thread's devices go.
    crate::arch::acpi_capsule_exit(pid);
    if !last_of_group(pid) {
        return true;
    }
    let holder = holder_of(pid);
    if !super::off_cpu::group_off_cpu(holder, current.then_some(pid), wait) {
        return false;
    }
    // Asked again after the wait: a thread started meanwhile keeps them.
    if !last_of_group(pid) {
        return true;
    }
    // The claims first: nothing more granted or entered, each device stopped
    // while its claim still names it; the claims go after the drains, once
    // no path is left inside them (L75).
    let claims = broker::claims_exit_begin(holder);
    broker::release_all_for_pid(holder, current);
    broker::irq_release_all_for_pid(holder);
    broker::dma_release_all_for_pid(holder, current);
    broker::pio_release_all_for_pid(holder);
    broker::claims_exit_end(holder, &claims);
    true
}

/// At teardown, once `pid` is a zombie: the CPU still running it is told to
/// stop before anything is given back, since it may run on, over frames and
/// a device the release hands on (L77).
pub(super) fn stop_then_release(pid: Pid, current: bool) {
    super::stop_elsewhere::stop_elsewhere(pid);
    let _ = release_if_last(pid, current, true);
}

/// No other thread of `pid`'s group is alive. A zombie no longer counts: it
/// runs nothing, and its own finalize finds this one gone.
pub(super) fn last_of_group(pid: Pid) -> bool {
    let threads = PROCESS_TABLE.get_all_processes().into_iter().map(|p| {
        let ended =
            matches!(*p.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_));
        (p.pid, p.thread_group_id(), !ended)
    });
    !others_alive(holder_of(pid), pid, threads)
}
