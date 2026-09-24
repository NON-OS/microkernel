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

//! System-wide totals, counted at the same sites as the per-process ones
//! plus the interrupt path, which has no process to charge.

use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Total {
    Syscalls = 0,
    IpcMessages = 1,
    Faults = 2,
    Interrupts = 3,
    /// Every switch onto the processor, preempted or yielded.
    Switches = 4,
    /// Ticks that landed in a process's own code, and in the kernel.
    UserTicks = 5,
    KernelTicks = 6,
}

const COUNT: usize = 7;

static TOTALS: [AtomicU64; COUNT] = [const { AtomicU64::new(0) }; COUNT];

#[derive(Clone, Copy, Default)]
pub struct Totals {
    pub syscalls: u64,
    pub ipc_messages: u64,
    pub faults: u64,
    pub interrupts: u64,
    pub switches: u64,
    pub user_ticks: u64,
    pub kernel_ticks: u64,
}

#[inline]
pub fn bump_total(total: Total) {
    TOTALS[total as usize].fetch_add(1, Ordering::Relaxed);
}

pub fn totals() -> Totals {
    let at = |t: Total| TOTALS[t as usize].load(Ordering::Relaxed);
    Totals {
        syscalls: at(Total::Syscalls),
        ipc_messages: at(Total::IpcMessages),
        faults: at(Total::Faults),
        interrupts: at(Total::Interrupts),
        switches: at(Total::Switches),
        user_ticks: at(Total::UserTicks),
        kernel_ticks: at(Total::KernelTicks),
    }
}
