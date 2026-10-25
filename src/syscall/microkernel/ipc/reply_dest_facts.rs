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

//! The facts reply_dest.rs rules on, read from the process table, the inbox
//! registry and the peer list, and a refusal said once, without a pid.

use super::reply_dest::{admit, Facts, Owner, Refused};
use crate::ipc::nonos_inbox::{owner_of, KERNEL_OWNER};
use crate::process::nonos_core::ProcessState;

static REFUSED: crate::sys::diag::Site = crate::sys::diag::Site::new(b"ipc.call");

/// Whether the answer `server` sends may go to `inbox`, the inbox the
/// pending entry recorded for `caller` when it called.
pub(super) fn admits(server: u32, inbox: &str, caller: u32) -> bool {
    let pcb = crate::process::get_process(caller);
    let alive = pcb.as_ref().is_some_and(|p| {
        !matches!(*p.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
    });
    let names_now = super::reply_inbox::for_pid(caller);
    let owner = match owner_of(inbox) {
        None => Owner::Missing,
        Some(KERNEL_OWNER) => Owner::Kernel,
        Some(pid) => Owner::Pid(pid),
    };
    let facts = Facts {
        recorded: inbox,
        names_now: &names_now,
        named: pcb.as_ref().and_then(|p| p.reply_inbox()).is_some(),
        owner,
        caller,
        alive,
        reachable: crate::services::registry::caller_may_reach_pid(caller),
    };
    let Err(refused) = admit(&facts) else { return true };
    let name = crate::process::with_process(server, |p| p.name()).unwrap_or_default();
    REFUSED.reply_refused(&name, why(refused));
    false
}

/// The answer was admitted and still not delivered: the caller's inbox was
/// full, or went with its owner after the check.
pub(super) fn undelivered(server: u32) {
    let name = crate::process::with_process(server, |p| p.name()).unwrap_or_default();
    REFUSED.reply_refused(&name, "the caller's inbox is full or gone");
}

const fn why(refused: Refused) -> &'static str {
    match refused {
        Refused::CallerGone => "the caller is gone",
        Refused::NotItsInbox => "the caller names another inbox",
        Refused::NotItsOwner => "the inbox is not the caller's",
        Refused::Unreachable => "the caller is off its peer list",
    }
}
