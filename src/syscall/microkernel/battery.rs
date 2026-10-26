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

//! `MkBatteryStatus` returns the battery charge as a percentage 0..=100,
//! or a negative errno saying why there is no number:
//!
//! - `ERRNO_NODEV`: the firmware declares no battery (no PNP0C0A device in
//!   the ACPI namespace). A desktop; the shell says "No battery".
//! - `ERRNO_NOTSUP`: a battery is declared, or the kernel cannot tell, but
//!   its charge cannot be read. The shell says "Battery status unavailable".
//!
//! A charge comes from evaluating the battery's `_BST` and `_BIX`/`_BIF`
//! methods, which read the embedded controller through an EmbeddedControl
//! OperationRegion. The kernel runs no AML; capsule_acpi does, and reports
//! what it read through MkAcpiPower. While that report is fresh it is the
//! answer; without one the kernel falls back to its own scan of the tables,
//! which can say a battery is absent but never what it holds. No number is
//! ever invented: a plausible fixed one is the one answer that cannot be
//! checked and cannot be corrected.

use super::errnos::{ERRNO_NODEV, ERRNO_NOTSUP};

crate::arch::acpi_only! {
    mod report;
}
crate::arch::acpi_only! {
    else
    #[path = "battery/report_absent.rs"]
    mod report;
}

pub fn sys_battery_status() -> i64 {
    if let Some(answer) = report::answer() {
        return answer;
    }
    if crate::arch::acpi_tables_declare_battery() == Some(false) {
        return ERRNO_NODEV;
    }
    ERRNO_NOTSUP
}
