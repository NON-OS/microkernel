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

//! The Wayland server's popups, the parts that are pure: the positioner,
//! where it places a popup, and the rules a popup's grab is held to.

#[path = "../../capsule_linux/src/linux/wayland/popup/axis.rs"]
pub mod axis;
#[path = "../../capsule_linux/src/linux/wayland/popup/place.rs"]
pub mod place;
#[path = "../../capsule_linux/src/linux/wayland/popup/positioner.rs"]
pub mod positioner;
#[path = "../../capsule_linux/src/linux/wayland/popup/rules.rs"]
pub mod rules;
#[path = "../../capsule_linux/src/linux/wayland/popup/side.rs"]
pub mod side;
#[path = "../../capsule_linux/src/linux/wayland/popup/stack.rs"]
pub mod stack;
