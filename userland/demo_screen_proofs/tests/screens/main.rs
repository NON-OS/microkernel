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

//! The windows a recording shows, at each display mode it may use
//! (`modes`), framed as app_skeleton frames them (`frame`), and written as
//! pictures to look at when DEMO_SHOTS names a directory (`png`). An
//! integration test, so the apps build as they ship, without cfg(test).

extern crate alloc;

mod about_shots;
mod app_census;
mod app_windows;
#[path = "../../../capsule_wm/src/server/handlers/window_open/cascade.rs"]
mod cascade;
mod compose;
#[allow(dead_code)] // the window manager's other placement constants
#[path = "../../../capsule_wm/src/server/handlers/window_open/constants.rs"]
mod constants;
mod desktop_shots;
mod frame;
#[path = "geometry_mount.rs"]
mod geometry;
mod modes;
mod png;
mod ring;
mod terminal_damage;
mod terminal_geometry;
mod terminal_shots;
mod terminal_tabs;
mod wallpaper_mount;
