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

//! Whether a process was spawned from the system's own namespace (L61), set
//! on its PCB before it can first run. The attesting root cannot say it: the
//! image's capsule set is proved under the vendor root, a store app's
//! included, so a check keyed on the root admitted a publisher's capsule as
//! the system's own.

use core::sync::atomic::Ordering;

/// A PCB flag bit: 63 is CONTINUED, 62 the released tables.
const SYSTEM_TIER: u64 = 1 << 61;

/// Mark `pid`, spawned from `systems.nonos`. Called once, before it runs.
pub(crate) fn mark_system(pid: u32) {
    let mark = |pcb: &alloc::sync::Arc<super::pcb::ProcessControlBlock>| {
        pcb.flags.fetch_or(SYSTEM_TIER, Ordering::AcqRel)
    };
    let _ = crate::process::with_process(pid, mark);
}

/// Whether `pid` was spawned from the system's own namespace.
pub(crate) fn is_system(pid: u32) -> bool {
    crate::process::with_process(pid, |pcb| pcb.flags.load(Ordering::Acquire) & SYSTEM_TIER != 0)
        .unwrap_or(false)
}
