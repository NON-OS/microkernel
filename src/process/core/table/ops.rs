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

use super::super::types::{Pid, ProcessState};
use super::types::ProcessTable;
use core::sync::atomic::Ordering;

impl ProcessTable {
    pub fn terminate_process(&self, pid: Pid) -> Result<(), &'static str> {
        /* Interrupts off while the write lock is held: see types.rs. */
        let removed = {
            let _irq = crate::interrupts::disable_interrupts_guard();
            let mut inner = self.inner.write();
            let pos = inner.iter().position(|p| p.pid == pid);
            pos.map(|pos| {
                *inner[pos].state.lock() = ProcessState::Terminated(0);
                inner.remove(pos)
            })
        };
        let Some(pcb) = removed else {
            return Err("Process not found");
        };
        /*
         * The last reference may be this one, and freeing it takes the heap
         * lock, so it is dropped here with interrupts back on.
         */
        drop(pcb);
        crate::sched::remove_from_run_queue(pid);
        /*
         * The registry states what is running, so a process that has stopped
         * must leave it or every later attestation overstates the machine.
         */
        crate::security::attest_registry::forget_attested(pid);
        Ok(())
    }

    pub fn set_process_group(&self, pid: Pid, pgid: Pid) -> Result<(), &'static str> {
        self.find_by_pid(pid)
            .map(|pcb| pcb.pgid.store(pgid, Ordering::Release))
            .ok_or("Process not found")
    }

    pub fn set_session_leader(&self, pid: Pid) -> Result<(), &'static str> {
        self.find_by_pid(pid)
            .map(|pcb| {
                pcb.sid.store(pid, Ordering::Release);
                pcb.pgid.store(pid, Ordering::Release);
            })
            .ok_or("Process not found")
    }
}
