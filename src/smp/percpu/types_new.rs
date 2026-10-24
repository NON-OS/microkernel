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

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64};

use super::types::{PerCpuData, ASID_NONE};

impl PerCpuData {
    pub const fn new() -> Self {
        Self {
            self_ptr: 0,
            cpu_id: 0,
            apic_id: 0,
            current_process: AtomicU64::new(0),
            current_thread: AtomicU64::new(0),
            kernel_stack_top: 0,
            user_stack_saved: 0,
            syscall_scratch: [0; 4],
            irq_nesting: 0,
            sched_lock_held: 0,
            random_state: AtomicU64::new(0),
            last_tick_tsc: AtomicU64::new(0),
            interrupt_disable_depth: 0,
            active_asid: AtomicU32::new(ASID_NONE),
            time_slice: AtomicU64::new(0),
            need_resched: AtomicU32::new(0),
            tlb_flush_pending: AtomicU32::new(0),
            accounting_idle: AtomicBool::new(false),
            tick_from_user: AtomicBool::new(false),
            pac: [const { AtomicU64::new(0) }; 5],
            _reserved: [0; 4096 - 176],
        }
    }
}
