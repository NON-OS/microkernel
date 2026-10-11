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

//! The PS/2 driver's scan set 1 tables, included so the tests can hold the
//! virtio keyboard to posting exactly what a PS/2 keyboard posts.

#[path = "../../capsule_driver_ps2_input/src/keymap/set1/mod.rs"]
pub mod set1;
#[path = "../../capsule_driver_ps2_input/src/keymap/set1_e0.rs"]
pub mod set1_e0;
