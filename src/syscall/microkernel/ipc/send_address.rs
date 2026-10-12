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
use crate::ipc::kernel_ipc::kernel_route_ipc_corr;
use crate::ipc::nonos_inbox::KERNEL_OWNER;
use crate::services::registry::lookup_service;
use crate::services::registry::{addressed_admits, caller_may_write_inbox, caller_owns};
use crate::syscall::caps::current_caps_or_default;
use crate::syscall::microkernel::errnos::ERRNO_PERM;

use super::send_trace::trace;

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

/*
 * Any other send goes to its addressed target with its own correlation
 * (0 for sys_ipc_send, all a forged reply injection can carry).
 */
pub(super) fn as_addressed(
    pid: u32,
    endpoint: u64,
    target: &str,
    data: &[u8],
    correlation: u64,
) -> i64 {
    if !super::send_caps::caller_satisfies_endpoint(endpoint, target)
        || !crate::services::registry::caller_may_reach(target)
        || !caller_may_address(target)
    {
        return ERRNO_PERM;
    }
    trace(pid, endpoint, target, data.len());
    match kernel_route_ipc_corr(pid, target, data, correlation) {
        Ok(()) => 0,
        Err(e) => e as i64,
    }
}
