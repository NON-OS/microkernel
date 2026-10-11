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

//! The owner's gates on an addressed send (services/registry/addressed.rs):
//! a send by any name of another capsule's is a write into its one inbox.

use crate::capabilities::caps_to_bits;
use crate::ipc::nonos_inbox::KERNEL_OWNER;
use crate::services::registry::lookup_service;
use crate::services::registry::{addressed_admits, caller_may_write_inbox, caller_owns};
use crate::syscall::caps::current_caps_or_default;

pub(super) fn caller_may_address(target: &str) -> bool {
    // An unknown name was already refused by its own gate.
    let Some(ep) = lookup_service(target) else {
        return false;
    };
    let held = caps_to_bits(&current_caps_or_default().permissions);
    let inbox_ok = || caller_may_write_inbox(ep.pid, held);
    if addressed_admits(ep.pid == KERNEL_OWNER, caller_owns(target), inbox_ok) {
        return true;
    }
    crate::log::warn!("[IPC-DENY] ipc to {}: its owner serves a gate the caller fails", target);
    false
}
