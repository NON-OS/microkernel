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

use crate::capabilities::caps_to_bits;
use crate::services::registry::caller_may_write_inbox;
use crate::syscall::caps::current_caps_or_default;

/// Whether this caller may write straight into `dest`'s own inbox with
/// MkIpcSendToPid. That inbox is where every endpoint `dest` serves is read,
/// so the send passes each one's gate, as a send by name would; checking
/// none of them let a capsule without Network reach net.sockets by its pid.
pub(super) fn caller_satisfies_inbox(dest: u32) -> bool {
    let held = caps_to_bits(&current_caps_or_default().permissions);
    if caller_may_write_inbox(dest, held) {
        return true;
    }
    let pid = crate::process::current_pid().unwrap_or(0);
    crate::log::warn!(
        "[CAP-DENY] pid={} ipc to pid {} fails a gate it serves, holds {:#x}",
        pid,
        dest,
        held
    );
    false
}
