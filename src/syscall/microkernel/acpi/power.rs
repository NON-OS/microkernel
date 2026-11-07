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

use crate::hardware::broker::acpi::power_report::{record, AC_KNOWN, BATTERY_PRESENT, KNOWN_BITS};
use crate::sys::bringup::{event, Ev};
use crate::syscall::microkernel::errnos::ERRNO_INVAL;

pub fn sys_acpi_power(report: u64) -> i64 {
    if report & !KNOWN_BITS != 0 {
        return ERRNO_INVAL;
    }
    let before = record(report, crate::sys::clock::since_boot_ms());
    trace(before, report);
    0
}

/// The battery and the adapter as bring-up events: on the first report,
/// and when either comes or goes.
fn trace(before: Option<u64>, now: u64) {
    let changed = |bit: u64| before.map_or(true, |b| (b ^ now) & bit != 0);
    if changed(BATTERY_PRESENT) {
        let ev = if now & BATTERY_PRESENT != 0 {
            Ev::Published
        } else {
            Ev::Absent("firmware reports none")
        };
        event("BATTERY", ev);
    }
    if changed(AC_KNOWN) {
        let ev =
            if now & AC_KNOWN != 0 { Ev::Published } else { Ev::Absent("no adapter state read") };
        event("AC", ev);
    }
}
