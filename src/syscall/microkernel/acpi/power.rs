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

//! `MkAcpiPower(report) -> 0`: the ACPI capsule's latest battery and AC
//! reading, as `power_report` lays the word out, kept for MkBatteryStatus.

use crate::hardware::broker::acpi::power_report::{record, KNOWN_BITS};
use crate::syscall::microkernel::errnos::ERRNO_INVAL;

pub fn sys_acpi_power(report: u64) -> i64 {
    if report & !KNOWN_BITS != 0 {
        return ERRNO_INVAL;
    }
    record(report, crate::sys::clock::since_boot_ms());
    0
}
