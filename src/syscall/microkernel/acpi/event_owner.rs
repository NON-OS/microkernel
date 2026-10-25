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

//! The MkAcpiEvent kinds only the ACPI capsule itself may send: that it
//! serves the fixed power button, and that it holds the Global Lock.

use super::super::event_act::say;
use crate::hardware::broker::acpi::button_owner::take;
use crate::hardware::broker::acpi::claim_holder::tgid;
use crate::hardware::broker::acpi::first_pass_rule::admits;
use crate::hardware::broker::acpi::leave_rule::lock_held;
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::ERRNO_PERM;

fn owner() -> Option<u32> {
    let owner = lookup_service("acpi").map(|e| e.pid);
    let caller = current_pid()?;
    admits(owner, caller, tgid).then(|| owner.unwrap_or(0))
}

/// The acpi owner's SCI serves the fixed power button now; the kernel's
/// poll stands down while that capsule lives (broker::acpi::button_owner).
/// 1 when a capsule served it before: this one is a restart.
pub(super) fn taken() -> i64 {
    let Some(owner) = owner() else { return ERRNO_PERM };
    let restart = take(owner);
    say(b"[ACPI] power button: served by the ACPI capsule; the kernel's poll stands down");
    i64::from(restart)
}

/// The Global Lock held or about to be given back, for the exit path.
pub(super) fn lock(held: bool) -> i64 {
    let Some(owner) = owner() else { return ERRNO_PERM };
    lock_held(owner, held);
    0
}
