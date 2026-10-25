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

//! The ACPI power meters (ACPI000D) on the port: POWER_METER (19) reads
//! them for the power service and the shell; SET_POWER_CAP (20) sets one's
//! hardware cap for the power service, only while `switch::WRITES` is on,
//! and it is off.

mod read;
mod set;
mod switch;

pub use read::power_meter;
pub use set::set_power_cap;
