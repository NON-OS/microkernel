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

use crate::smp::state::{AP_STARTUP_BARRIER, CPUS_ONLINE, CPU_DESCRIPTORS};
use crate::smp::types::CpuState;
use core::sync::atomic::Ordering;

/*
 * Online is published after `sti`, not before, because Online is what
 * makes this CPU a target. `shootdown::broadcast` selects targets with
 * `cpu_is_online` and then waits for each to acknowledge by interrupt.
 * Declaring Online while interrupts are still masked offers the rest of
 * the machine a CPU that is required to answer and cannot.
 *
 * The window is not theoretical and it is not wide by accident. The boot
 * CPU's `wait_online` returns the moment it sees this flag, and the next
 * thing it does is map the following AP's stack, which flushes, which
 * broadcasts. The target is whichever AP just set this.
 *
 * It stayed hidden because the population was published as a count by the
 * boot CPU after every AP had started, so `cpus_online()` read 1 for the
 * whole of bring-up and every one of these shootdowns was skipped.
 */
pub(super) fn publish(cpu_id: u32) {
    CPU_DESCRIPTORS[cpu_id as usize].set_state(CpuState::Online);
    CPU_DESCRIPTORS[cpu_id as usize].set_stage(crate::smp::Stage::Online);
    CPUS_ONLINE.fetch_add(1, Ordering::AcqRel);
    AP_STARTUP_BARRIER.fetch_add(1, Ordering::Release);
}
