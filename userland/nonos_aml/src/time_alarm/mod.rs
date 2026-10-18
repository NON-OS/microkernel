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

//! The Time and Alarm device (ACPI000E): a real-time clock firmware keeps,
//! and wake timers on AC and on DC power. Read from its methods; the two
//! writes (the time, a wake alarm) are here too, and whoever calls them
//! decides whether they are allowed. The method set and the layouts are
//! recalled from ACPI 6.5, 9.18, not read: unverified.

mod find;
mod read;
mod time;
mod write;

pub use find::{time_alarm_device, TIME_ALARM_HID};
pub use read::{read, TimeAlarm, Timer, AC, DC, DISABLED};
pub use time::{RealTime, TZ_UNSPECIFIED};
pub use write::{set_alarm, set_time, WriteError};
