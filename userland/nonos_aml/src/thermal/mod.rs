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

mod temp;
mod trips;
mod verdict;
mod zones;

pub use temp::{milli_celsius, tenths_kelvin, COLDEST, HOTTEST};
pub use trips::{active_seg, temperature, trips, Trips, MAX_ACTIVE};
pub use verdict::{judge, Verdict, HYSTERESIS_MC};
pub use zones::{active_devices, zones};
