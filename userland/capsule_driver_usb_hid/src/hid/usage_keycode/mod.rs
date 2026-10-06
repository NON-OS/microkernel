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

//! The code a USB keyboard usage posts when it is not a character key: the
//! navigation, function and lock keys, the keypad, and the volume and power
//! keys.
//!
//! The codes are the PS/2 driver's keycode table
//! (capsule_driver_ps2_input keymap/set1/keycodes.rs), which app_skeleton's
//! KEY_* constants and every app read. This driver posted its own 0xE000 page
//! for the arrows, Home, End, Page Up, Page Down and Delete, and raw usages
//! for F1 to F12 and Insert, so on a USB keyboard none of them did anything
//! in any app. input_proofs holds the two tables to each other.

mod codes;
mod map;

pub use map::usage_keycode;
