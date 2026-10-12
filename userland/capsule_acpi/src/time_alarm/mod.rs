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

//! The Time and Alarm device (ACPI000E) on the port: TIME_ALARM (14) reads
//! it for the power service and the shell; TIME_SET (15) sets the clock
//! or a wake alarm for the power service, only while `switch::WRITES` is
//! on, and it is off.

mod read;
mod set;
mod switch;

pub use read::time_alarm;
pub use set::time_set;
