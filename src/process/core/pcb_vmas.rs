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

//! The lock that guards a process's VMA list.

use spin::MutexGuard;

use super::pcb::ProcessControlBlock;
use super::types::MemoryState;

impl ProcessControlBlock {
    /// The address-space bookkeeping, taken without going deaf to TLB
    /// shootdowns.
    ///
    /// Every system call runs with interrupts masked, so a CPU spinning here
    /// with `lock()` could not acknowledge a shootdown aimed at the address
    /// space it is running, and a holder waiting on that acknowledgement
    /// would never release. Nothing may change a page table while holding
    /// this guard either: map and unmap with it dropped.
    pub fn memory_state(&self) -> MutexGuard<'_, MemoryState> {
        crate::smp::lock_responsive(&self.memory)
    }
}
