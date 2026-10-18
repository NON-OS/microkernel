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

//! `MkAcpiOscGrant(segment, first << 8 | last, offered, granted)`: the ACPI
//! capsule publishes what a PCI root bridge's _OSC offered and what NONOS
//! committed, for the kernel's own readers (`osc_store::osc_grant`). Only
//! the `acpi` service's owner may call it. EINVAL for buses backwards, a
//! word past 32 bits, or a grant past the offer; ENOSPC when full. A grant
//! past the kernel's own claim is recorded cut to it (F-110b).

use core::sync::atomic::{AtomicBool, Ordering};

use crate::hardware::broker::acpi::osc_grant::{checked, within_claim, OscGrant, KERNEL_CLAIM};
use crate::hardware::broker::acpi::osc_store::publish;
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOSPC, ERRNO_PERM};
use crate::sys::serial::Line;

/// The extra said once: a firmware that over-grants does it at every boot.
static SAID: AtomicBool = AtomicBool::new(false);

pub fn sys_acpi_osc_grant(segment: u64, buses: u64, offered: u64, granted: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    let words = (u16::try_from(segment), u16::try_from(buses));
    let (Ok(segment), Ok(buses)) = words else { return ERRNO_INVAL };
    let (Ok(offered), Ok(granted)) = (u32::try_from(offered), u32::try_from(granted)) else {
        return ERRNO_INVAL;
    };
    let [last, first] = buses.to_le_bytes();
    let Some(g) = checked(OscGrant { segment, first, last, offered, granted }) else {
        return ERRNO_INVAL;
    };
    // The kernel's own claim bounds what it records, not the capsule's mask.
    let (g, extra) = within_claim(g, KERNEL_CLAIM);
    if extra != 0 && !SAID.swap(true, Ordering::Relaxed) {
        let line = b"[ACPI] _OSC granted controls past the kernel's claim, not taken: ";
        Line::new().str(line).hex(u64::from(extra)).end();
    }
    if publish(g) {
        0
    } else {
        ERRNO_NOSPC
    }
}
