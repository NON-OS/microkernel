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

//! `MkAcpiLpit(index, field)`: LPIT's `index`th low-power idle state, one
//! field at a time, for the ACPI capsule's suspend-to-idle. Read-only.
//! Field 0: the unique id, with flags << 16 (bit 0 disabled, bit 1 has a
//! residency counter); 1: the minimum residency and 2: the wake latency,
//! microseconds; 3: the counter's frequency (0, the TSC's); 4: the counter
//! now. Only the `acpi` service's owner may call it. ENOENT past the last
//! state or for a state with no counter; EINVAL for a field above 4;
//! ENOTSUP for a counter in a space or register the kernel will not read.

use crate::hardware::broker::acpi::lpit_store::{lpit_residency, lpit_state, CounterError};
use crate::process::current_pid;
use crate::services::registry::lookup_service;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_NOTSUP, ERRNO_PERM};

pub fn sys_acpi_lpit(index: u64, field: u64) -> i64 {
    let owner = lookup_service("acpi").map(|e| e.pid);
    if owner.is_none() || owner != current_pid() {
        return ERRNO_PERM;
    }
    let Ok(index) = usize::try_from(index) else { return ERRNO_NOENT };
    let Some(s) = lpit_state(index) else { return ERRNO_NOENT };
    let wide = |v: u64| i64::try_from(v).unwrap_or(i64::MAX);
    match field {
        0 => {
            let flags = u64::from(s.disabled) | u64::from(s.counter.is_some()) << 1;
            (u64::from(s.id) | flags << 16) as i64
        }
        1 => i64::from(s.min_residency_us),
        2 => i64::from(s.latency_us),
        3 => s.counter.map_or(ERRNO_NOENT, |c| wide(c.frequency)),
        4 => match lpit_residency(index) {
            Ok(v) => wide(v),
            Err(CounterError::Unreadable) => ERRNO_NOTSUP,
            Err(_) => ERRNO_NOENT,
        },
        _ => ERRNO_INVAL,
    }
}
