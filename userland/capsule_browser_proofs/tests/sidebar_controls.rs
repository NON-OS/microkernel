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

//! The sidebar's rows, drawn as shared `NavItem`s under a `SectionHeader`
//! and laid out in one `Controls`, and the geometry that places them, at 1x
//! and 2x: each row lights and acts only where it is drawn, the row on show
//! is the only one marked, and a full window's New group takes nothing.

#[path = "sidebar_controls/geom_tests.rs"]
mod geom_tests;
#[path = "sidebar_controls/kit.rs"]
mod kit;
#[path = "sidebar_controls/look_tests.rs"]
mod look_tests;
#[path = "sidebar_controls/press_tests.rs"]
mod press_tests;
