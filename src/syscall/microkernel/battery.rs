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
//! OperationRegion. That needs an AML interpreter, and the kernel does not
//! have one: it scans AML for device declarations and constant packages and
//! never executes it. So no percentage is ever returned today, and none is
//! invented: a plausible fixed number is the one answer that cannot be
//! checked and cannot be corrected.

use super::errnos::ERRNO_NOTSUP;

pub fn sys_battery_status() -> i64 {
    #[cfg(target_arch = "x86_64")]
    {
        if crate::arch::x86_64::acpi::parser::with_data(|d| d.power_devices.battery) == Some(false)
        {
            return super::errnos::ERRNO_NODEV;
        }
    }
    ERRNO_NOTSUP
}
