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

//! The desktop's own capsules: the compositor, `wm` and the
//! wallpaper with its catalogue.

#[path = "capsule_compositor/mod.rs"]
pub mod capsule_compositor;
#[path = "capsule_wallpaper/mod.rs"]
pub mod capsule_wallpaper;
#[path = "capsule_wallpaper_catalog/mod.rs"]
pub mod capsule_wallpaper_catalog;
#[path = "capsule_wm/mod.rs"]
pub mod capsule_wm;
