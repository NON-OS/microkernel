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

pub(super) fn release_if_last(pid: Pid, current: bool) {
    if !last_of_group(pid) {
        return;
    }
    let holder = holder_of(pid);
    broker::release_all_for_pid(holder, current);
    broker::irq_release_all_for_pid(holder);
    broker::dma_release_all_for_pid(holder, current);
    broker::pio_release_all_for_pid(holder);
}

/// No other thread of `pid`'s group is alive. A zombie no longer counts: it
/// runs nothing, and its own finalize finds this one gone.
fn last_of_group(pid: Pid) -> bool {
    let threads = PROCESS_TABLE.get_all_processes().into_iter().map(|p| {
        let ended =
            matches!(*p.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_));
        (p.pid, p.thread_group_id(), !ended)
    });
    !others_alive(holder_of(pid), pid, threads)
}
