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

use crate::ipc::nonos_inbox::{self, StrictEnqueueError, KERNEL_OWNER};
use crate::process::caps;
use crate::services::registry::{lookup_service, routes, ServiceEndpoint};

pub const EACCES: i32 = -13;
pub const ENOENT: i32 = -2;
pub const ESRCH: i32 = -3;
pub const ENOMEM: i32 = -12;
pub const EAGAIN: i32 = -11;

/*
 * The route's capability check, by the rule every gate of a send applies
 * (services/registry/send_rule.rs routes). It is the last before the message
 * is queued, and a refusal here is said: the gates before it admitted the
 * send, so a refusal means they and the route disagree.
 */
fn route_admits(caller_pid: u32, target: &str, endpoint: &ServiceEndpoint) -> bool {
    let held = caps::bits(caller_pid);
    if routes(target, endpoint.caps_required, held) {
        return true;
    }
    crate::log::warn!(
        "[ROUTE-DENY] pid={} ipc to {} needs caps {:#x}, holds {:#x}",
        caller_pid,
        target,
        endpoint.caps_required,
        held.unwrap_or(0)
    );
    false
}

pub fn kernel_check_ipc_permission(caller_pid: u32, target: &str) -> Result<(), i32> {
    let endpoint = lookup_service(target).ok_or(ENOENT)?;
    if !route_admits(caller_pid, target, &endpoint) {
        return Err(EACCES);
    }
    Ok(())
}

pub fn kernel_route_ipc(caller_pid: u32, target: &str, data: &[u8]) -> Result<(), i32> {
    kernel_route_ipc_corr(caller_pid, target, data, 0)
}

pub fn kernel_route_ipc_corr(
    caller_pid: u32,
    target: &str,
    data: &[u8],
    correlation: u64,
) -> Result<(), i32> {
    let endpoint = lookup_service(target).ok_or(ENOENT)?;
    if !route_admits(caller_pid, target, &endpoint) {
        return Err(EACCES);
    }
    // Capsule services route into the owning process inbox. Kernel-owned
    // reply endpoints route to the endpoint inbox the kernel client drains.
    let dest = if endpoint.pid == KERNEL_OWNER {
        alloc::string::String::from(target)
    } else {
        alloc::format!("proc.{}", endpoint.pid)
    };
    let mut msg = crate::ipc::nonos_channel::IpcMessage::new(
        &alloc::format!("proc.{}", caller_pid),
        &dest,
        data,
    )
    .map_err(|_| ENOMEM)?;
    msg.correlation = correlation;
    match nonos_inbox::try_enqueue_strict(&dest, msg) {
        Ok(()) => {
            // Wake on delivery (state-checked, idempotent): the sleep-map
            // gate strands a Sleeping receiver that has no map entry.
            if endpoint.pid != KERNEL_OWNER {
                crate::sched::wake_process(endpoint.pid);
            }
            Ok(())
        }
        Err(StrictEnqueueError::MissingInbox) | Err(StrictEnqueueError::DeadOwner) => Err(ESRCH),
        Err(StrictEnqueueError::QueueFull(_)) => Err(EAGAIN),
    }
}
