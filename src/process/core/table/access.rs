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
use super::types::ProcessTable;
use alloc::{sync::Arc, vec::Vec};

impl ProcessTable {
    // Masked, because the timer walks this table: boot inserts with interrupts
    // on, and a tick taking the read side on this cpu would spin forever.
    pub fn add(&self, pcb: Arc<ProcessControlBlock>) {
        let _irq = crate::interrupts::disable_interrupts_guard();
        self.inner.write().push(pcb);
    }
    pub fn get_all_processes(&self) -> Vec<Arc<ProcessControlBlock>> {
        self.inner.read().clone()
    }
    pub fn find_by_pid(&self, pid: Pid) -> Option<Arc<ProcessControlBlock>> {
        self.inner.read().iter().find(|p| p.pid == pid).cloned()
    }
    /// The blocks of the pids in `sorted`, ascending, in one pass over the
    /// table instead of one lookup per pid.
    pub fn find_sorted(&self, sorted: &[Pid]) -> Vec<Arc<ProcessControlBlock>> {
        let table = self.inner.read();
        table.iter().filter(|p| sorted.binary_search(&p.pid).is_ok()).cloned().collect()
    }
    /// `find_by_pid` for a caller that must not wait: None when the table is
    /// being written, Some(None) when the pid is not in it.
    pub fn try_find_by_pid(&self, pid: Pid) -> Option<Option<Arc<ProcessControlBlock>>> {
        Some(self.inner.try_read()?.iter().find(|p| p.pid == pid).cloned())
    }
    pub fn is_active_name(&self, name: &str) -> bool {
        self.inner.read().iter().any(|p| p.name.lock().as_str() == name)
    }
    pub fn is_active_pid(&self, pid: u64) -> bool {
        self.inner.read().iter().any(|p| p.pid as u64 == pid)
    }
    pub fn get_children_of(&self, parent_pid: Pid) -> Vec<Arc<ProcessControlBlock>> {
        self.inner.read().iter().filter(|p| p.parent_pid() == parent_pid).cloned().collect()
    }
    pub fn get_process(&self, pid: Pid) -> Option<Arc<ProcessControlBlock>> {
        self.find_by_pid(pid)
    }
}
