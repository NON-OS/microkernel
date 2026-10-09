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

//! The driver's scan set 1 decode, at the `crate::keymap` paths its files name
//! each other by: the base table, the E0 table, the decode that picks one by
//! the prefix and reads the break bit, and the keys whose repeats it drops.

#[path = "../../capsule_driver_ps2_input/src/keymap/set1/mod.rs"]
pub mod set1;
#[path = "../../capsule_driver_ps2_input/src/keymap/set1_e0.rs"]
pub mod set1_e0;
#[path = "../../capsule_driver_ps2_input/src/keymap/translate.rs"]
pub mod translate;
#[path = "../../capsule_driver_ps2_input/src/keymap/once.rs"]
pub mod once;
#[path = "../../capsule_driver_ps2_input/src/keymap/keypad.rs"]
pub mod keypad;
