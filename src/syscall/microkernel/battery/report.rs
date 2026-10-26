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

//! The ACPI capsule's battery report, while it is fresh: the answer, or None
//! for the caller to fall back to the tables' scan.

use crate::hardware::broker::acpi::power_report::{battery, Battery};
use crate::syscall::microkernel::errnos::{ERRNO_NODEV, ERRNO_NOTSUP};

pub(super) fn answer() -> Option<i64> {
    match battery(crate::sys::clock::since_boot_ms()) {
        Battery::Percent(p) => Some(p as i64),
        Battery::Absent => Some(ERRNO_NODEV),
        Battery::Present => Some(ERRNO_NOTSUP),
        Battery::Unreported => None,
    }
}
