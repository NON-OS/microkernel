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

//! The Wayland server's selections, the parts that are pure: what a guest
//! holds, the offers its devices are given, a selection replaced, and the
//! text types the desktop clipboard is carried as. They name the server's
//! objects, events and numbers as their own module's, so those are here.

pub use crate::ops::{data, select};
pub use crate::out::Event;
pub use crate::wayland_object::{Object, Objects};

#[path = "../../capsule_linux/src/linux/wayland/clip/gone.rs"]
pub mod gone;
#[path = "../../capsule_linux/src/linux/wayland/clip/mime.rs"]
pub mod mime;
#[path = "../../capsule_linux/src/linux/wayland/clip/offer.rs"]
pub mod offer;
#[path = "../../capsule_linux/src/linux/wayland/clip/set.rs"]
pub mod set;
#[path = "../../capsule_linux/src/linux/wayland/clip/sources.rs"]
pub mod sources;
#[path = "../../capsule_linux/src/linux/wayland/clip/state.rs"]
pub mod state;
#[path = "../../capsule_linux/src/linux/wayland/clip/which.rs"]
pub mod which;
