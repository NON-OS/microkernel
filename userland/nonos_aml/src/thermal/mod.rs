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

//! Thermal zones (ACPI 6.5, chapter 11): their temperature, their trip
//! points, and what a reading calls for.

mod passive;
mod passive_rule;
mod temp;
mod trip_more;
mod trip_read;
mod trip_rule;
mod trips;
mod verdict;
mod watch;
mod zones;

pub use passive::{
    constants, passive_constants, passive_devices, step, Constants, FULL, MAX_TC, MAX_TSP,
};
pub use temp::{milli_celsius, tenths_kelvin, COLDEST, HOTTEST};
pub use trip_more::{UNREAD_AC, UNREAD_TRIPS, UNREAD_TZP};
pub use trip_read::{Read, UNREAD_CRT, UNREAD_HOT, UNREAD_PSV};
pub use trip_rule::{sane, CRT_UNDER_PSV, HOT_OVER_CRT, HOT_UNDER_PSV, HOT_UNDER_PSV_KEPT};
pub use trips::{active_seg, temperature, trips, Trips, MAX_ACTIVE};
pub use verdict::{judge, Verdict, HYSTERESIS_MC};
pub use watch::{Watch, CONFIRM_MS, CRITICAL_POLL_MS, UNREAD_TMP};
pub use zones::{active_devices, zones};
