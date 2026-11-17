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

//! The toolbar's controls, drawn from shared widgets and laid out in one
//! `Controls`, at 1x and 2x: each lights and acts only where it is drawn, a
//! disabled one never acts, and Reload, the star and Reader show their state.

#[path = "toolbar_controls/field_tests.rs"]
mod field_tests;
#[path = "toolbar_controls/kit.rs"]
mod kit;
#[path = "toolbar_controls/look_tests.rs"]
mod look_tests;
#[path = "toolbar_controls/press_tests.rs"]
mod press_tests;
#[path = "toolbar_controls/spots.rs"]
mod spots;
