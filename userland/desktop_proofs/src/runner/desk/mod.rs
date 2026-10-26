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

//! A desktop in one process: the window manager's table, z stack and focus,
//! the compositor's scene and damage, the input router's press grab, and an
//! app_skeleton frame per window. Each method carries one step of the pointer
//! path the way the capsules carry it over IPC, calling the real code at every
//! hop; the glue between the hops is the order the capsules call them in.

mod band;
mod close;
mod consistent;
mod consts;
mod deliver;
mod dock;
mod event;
mod full_screen;
mod open;
mod open_app;
mod paint;
mod place;
mod press;
mod resize;
mod shell_sync;
mod types;
mod wm;

pub use consts::DOCK_WINDOW_ID;
pub use event::event;
pub use types::Desk;
