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

//! `MkAcpiFirstPass() -> 0`: the ACPI capsule says its first publish pass
//! is done (`broker::acpi::first_pass`). Only the `acpi` service's owner's
//! thread group may say so; EPERM for anyone else. Saying it again does
//! nothing, and nothing ever clears it.

use crate::hardware::broker::acpi::claim_holder::tgid;
use crate::hardware::broker::acpi::first_pass::mark;
use crate::hardware::broker::acpi::first_pass_rule::admits;
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::sys::serial::Line;
use crate::syscall::microkernel::errnos::ERRNO_PERM;

pub fn sys_acpi_first_pass() -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    let Some(caller) = current_pid() else { return ERRNO_PERM };
    if !admits(owner, caller, tgid) {
        return ERRNO_PERM;
    }
    if mark() {
        Line::new().str(b"[ACPI] first publish pass done").end();
    }
    0
}
