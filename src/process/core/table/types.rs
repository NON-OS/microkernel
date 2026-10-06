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

//! Every process the kernel knows, by pid.

use super::super::pcb::ProcessControlBlock;
use alloc::{sync::Arc, vec::Vec};
use spin::RwLock;

/*
 * The timer tick reads this table (waking sleepers, and giving a woken High
 * process a CPU), so a writer holds interrupts off for as long as it has the
 * lock. A tick landing on the CPU that holds the write lock would otherwise
 * spin on a lock the interrupted code can never release, and the other CPUs
 * would pile up behind it: a spawn and an exit at the same moment froze a
 * 4 CPU desktop that way.
 */
#[derive(Default)]
pub struct ProcessTable {
    pub(super) inner: RwLock<Vec<Arc<ProcessControlBlock>>>,
}

pub static PROCESS_TABLE: ProcessTable = ProcessTable { inner: RwLock::new(Vec::new()) };
