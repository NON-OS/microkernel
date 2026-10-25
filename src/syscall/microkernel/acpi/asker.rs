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

//! How the ACPI holder calls name who is asking: a mode word, 0 for the
//! capsule's own look (the pid must then be 0) and 1 for a driver's
//! request (the pid must then be that driver's), so no pid value means
//! anything by itself.

use crate::hardware::broker::acpi::claim_holder::{tgid, Asker};
use crate::hardware::broker::acpi::first_pass_rule::admits;
use crate::process::current_pid;
use crate::services::registry::lookup_service;

#[path = "owner_gate.rs"]
pub(super) mod owner_gate;

const LOOK: u64 = 0;
const HOLDER: u64 = 1;

/// A question about whether another process holds a device is the acpi
/// owner's alone: anyone else could map who holds what (F-95).
pub(super) fn asker(mode: u64, pid: u64) -> Option<Asker> {
    match (mode, u32::try_from(pid).ok()?) {
        (LOOK, 0) => Some(Asker::Anyone),
        (HOLDER, pid) if pid != 0 && caller_is_owner() => Some(Asker::Holder(pid)),
        _ => None,
    }
}

/// The caller runs in the acpi owner's thread group.
pub(super) fn caller_is_owner() -> bool {
    let owner = lookup_service("acpi").map(|e| e.pid);
    current_pid().is_some_and(|caller| admits(owner, caller, tgid))
}
