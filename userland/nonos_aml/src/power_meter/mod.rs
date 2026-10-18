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

//! The ACPI power meter (ACPI000D, ACPI 6.5 10.4): what it measures and
//! what it reads now, read-only; setting the hardware cap is the one
//! write, and the capsule keeps it behind a switch.

mod cap;
mod pmc;
mod read;

pub use cap::{set_cap, CapError};
pub use pmc::{parse_pmc, Pmc};
pub use read::{meters, read_meter, Meter, POWER_METER_HID};
