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
//! or a negative errno when no battery can be read.
//!
//! There is no reading to give. A battery percentage comes from the ACPI
//! `_BST` and `_BIF` objects, evaluating those needs an AML interpreter, and
//! the kernel does not have one: it scans AML for device resources and never
//! executes it.
//!
//! So this refuses. It previously answered a fixed 100, which meant every
//! caller was told the machine was on full charge, on hardware with no battery
//! at all, with no way to tell that apart from a real reading. A shell asking
//! for a number it cannot have should be told so, and the caller already
//! handles a negative return as "not reported". Inventing a plausible number
//! is the one answer that cannot be checked and cannot be corrected.
//!
//! When an AML evaluator lands, replace the body with the real remaining
//! capacity and this comment with nothing.

use super::errnos::ERRNO_NODEV;

pub fn sys_battery_status() -> i64 {
    ERRNO_NODEV
}
