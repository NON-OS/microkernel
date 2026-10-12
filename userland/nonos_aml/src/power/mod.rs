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

//! Batteries and AC adapters (ACPI 6.5, 10.2 and 10.3): their packages
//! decoded, and the charge they amount to.

mod bix;
pub(crate) mod fields;
mod info;
mod status;
mod time;

pub use bix::parse_bix;
pub use info::{parse_bif, BatteryInfo};
pub use status::{online, parse_bst, percent, BatteryStatus};
pub use time::minutes_left;
