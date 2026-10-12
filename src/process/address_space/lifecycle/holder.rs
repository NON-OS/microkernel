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

use alloc::sync::Arc;
use core::sync::atomic::Ordering;

use crate::process::core::ProcessControlBlock;

/// Another thread still on `pcb`'s tables, which then pass to it.
pub(crate) fn holder_after(pcb: &ProcessControlBlock) -> Option<Arc<ProcessControlBlock>> {
    let tables = pcb.cr3.load(Ordering::Acquire);
    if tables == 0 {
        return None;
    }
    crate::process::core::PROCESS_TABLE
        .get_all_processes()
        .into_iter()
        .find(|p| p.pid != pcb.pid && p.cr3.load(Ordering::Acquire) == tables)
}
