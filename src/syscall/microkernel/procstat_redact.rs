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

//! What `MkProcStat` shows a caller about processes other than itself.
//!
//! Any valid token may call it, so it must say less than `MkAttestEntries`,
//! which needs AttestRead. Another process's capability mask is exactly what
//! that call gates, and its live counters are a timing channel: the IPC count
//! of the focused app rises with each keystroke. A caller without AttestRead
//! or ProcessControl sees another process's identity and state, and nothing it
//! does.

use super::procstat_entry::ProcStatEntry;
use crate::capabilities::Capability;

/// Whether the calling process may read every field of every entry.
pub(super) fn sees_all() -> bool {
    let token = crate::syscall::caps::current_caps_or_default();
    token.is_valid()
        && (token.grants(Capability::AttestRead) || token.grants(Capability::ProcessControl))
}

/// `e` as the caller may see it.
pub(super) fn visible(mut e: ProcStatEntry, caller: u32, all: bool) -> ProcStatEntry {
    if all || e.pid == caller {
        return e;
    }
    e.run_ticks = 0;
    e.caps = 0;
    e.mem_kb = 0;
    e.syscalls = 0;
    e.ipc_tx = 0;
    e.ipc_rx = 0;
    e.faults = 0;
    e.switches = 0;
    e.user_ticks = 0;
    e.mapped_kb = 0;
    e.vma_count = 0;
    e
}
