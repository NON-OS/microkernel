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

//! `MkAcpiThermalLimit(uid, percent) -> 0`: the ACPI capsule sets the
//! ceiling passive cooling allows the processor the MADT names by this ACPI
//! processor UID, in percent of full performance; 100 lifts it. Owner only
//! (F-95's gate). EINVAL past 100; ENOSPC once every processor slot holds
//! a limit. Nothing acts on the ceiling until track A's frequency control
//! reads it; the critical trip still powers the machine off.

use crate::hardware::broker::acpi::cpu_store::thermal_limit::{rule::Placed, set};
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOSPC};

pub fn sys_acpi_thermal_limit(uid: u64, pct: u64) -> i64 {
    match set(uid, pct) {
        Placed::Bad => ERRNO_INVAL,
        Placed::Full => ERRNO_NOSPC,
        Placed::Set | Placed::Cleared => {
            crate::log_info!("[ACPI] cpu {}: thermal limit {}%", uid, pct);
            0
        }
        Placed::Same => 0,
    }
}
