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
//! asked for by a power-button press, a thermal zone at _CRT or a critical
//! battery, then run from the capsule's loop with \_TTS(5) and \_PTS(5)
//! first (`serve`, F-111). Firmware's own critical shutdown would cut power
//! without the wipe. The shell's Power Off is not served here: `power.off`
//! has one owner, capsule_power, which asks this capsule to prepare
//! soft-off (OP_PREPARE_SLEEP) before its own AdminShutdown (P-08).

mod button;
pub mod rule;
mod serve;

pub use button::{policy, press, take_button, Presses};

pub use serve::{power_off, serve, Why};
