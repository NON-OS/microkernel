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

//! The Wayland server's subsurfaces, the parts that are pure: the blend,
//! the painting order, what is kept, and what a parent's commit applies.

#[path = "../../capsule_linux/src/linux/wayland/subsurface/blend.rs"]
pub mod blend;
#[path = "../../capsule_linux/src/linux/wayland/subsurface/order.rs"]
pub mod order;
#[path = "../../capsule_linux/src/linux/wayland/subsurface/restack.rs"]
pub mod restack;
#[path = "../../capsule_linux/src/linux/wayland/subsurface/state.rs"]
pub mod state;
