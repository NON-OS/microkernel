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

use super::super::pcb::ProcessControlBlock;
use super::super::types::Pid;
use alloc::{sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicU32, Ordering};

use super::current_pid::CurrentPid;
use spin::RwLock;

#[derive(Default)]
pub struct ProcessTable {
    pub(super) inner: RwLock<Vec<Arc<ProcessControlBlock>>>,
}

pub static PROCESS_TABLE: ProcessTable = ProcessTable { inner: RwLock::new(Vec::new()) };
pub static CURRENT_PID: CurrentPid = CurrentPid::new();
pub(super) static NEXT_PID: AtomicU32 = AtomicU32::new(1);

static PID_ALLOC_LOCK: spin::Mutex<()> = spin::Mutex::new(());

pub fn allocate_tid() -> Option<Pid> {
    // The single serialized PID allocator.
    let _guard = PID_ALLOC_LOCK.lock();
    let current = NEXT_PID.load(Ordering::SeqCst);
    match super::pid_alloc::choose_pid(current, |p| PROCESS_TABLE.is_active_pid(p as u64)) {
        Some((pid, next)) => {
            NEXT_PID.store(next, Ordering::SeqCst);
            crate::process::exit::purge_for_new_pid(pid);
            Some(pid)
        }
        None => {
            crate::log::error!("[PROCESS] PID space exhausted");
            None
        }
    }
}
