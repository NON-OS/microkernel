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

//! The wiped power-off (F-88). AdminShutdown wipes on the way down; it is
//! asked for by a power-button press, a thermal zone at _CRT, a critical
//! battery, or the shell's POFF frame on `acpi` (the shell confirms first;
//! nothing here asks; `power.off` is capsule_power's). Each is asked for, then run from the capsule's
//! loop with \_TTS(5) and \_PTS(5) first (`serve`, F-111). Firmware's own
//! critical shutdown would cut power without the wipe.

mod button;
mod request;
mod serve;
pub mod rule;
pub mod wire;

pub use button::{policy, press, take_button, Presses};
pub use request::requested;

pub use serve::{power_off, serve, Why};
