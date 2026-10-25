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

//! The prediction barrier between processes on one CPU.

use core::sync::atomic::{AtomicU32, Ordering};

use super::constants::{MSR_IA32_PRED_CMD, PRED_CMD_IBPB};
use super::cpu_apply::has_ibpb;
use super::msr::wrmsr;
use super::policy::barrier_on_switch;
use super::rsb::rsb_fill;
use crate::smp::MAX_CPUS;

/// The thread group each CPU last switched to; 0 before its first switch.
static LAST_GROUP: [AtomicU32; MAX_CPUS] = [const { AtomicU32::new(0) }; MAX_CPUS];

/// Before this CPU runs thread group `next`: IBPB, and a refilled return
/// stack buffer, when it last ran a different process.
pub fn switch_barrier(next: u32) {
    let cpu = crate::smp::cpu_id();
    let prev = LAST_GROUP.get(cpu).map_or(0, |last| last.swap(next, Ordering::Relaxed));
    if barrier_on_switch(has_ibpb(), prev, next) {
        // SAFETY: `has_ibpb` means CPUID reported IBRS/IBPB, so PRED_CMD exists.
        unsafe { wrmsr(MSR_IA32_PRED_CMD, PRED_CMD_IBPB) };
        rsb_fill();
    }
}

/// The switch `switch_barrier` was called for did not happen, and this CPU
/// goes back to what it ran: forget the group just recorded, so the next
/// switch to any group takes the barrier. Recorded, a process that ran here
/// in between went unbarriered against the one that followed it.
pub fn switch_aborted() {
    if let Some(last) = LAST_GROUP.get(crate::smp::cpu_id()) {
        last.store(0, Ordering::Relaxed);
    }
}
