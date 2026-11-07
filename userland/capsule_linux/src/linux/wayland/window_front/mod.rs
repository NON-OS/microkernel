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
//! A guest's toplevels, each a Window of its own (scene_window.rs) with its
//! own compositor layer (present/layer.rs) and window manager window, all
//! on screen at once. The one being acted on is `Scene.win`; the others
//! are `Scene.behind`. Whatever acts on a window brings it to `win` first
//! (select.rs): a commit by its surface, a routed pointer event by the
//! window it names, a key by the window with the keyboard, a window
//! manager notice by its window id. A toplevel mapped for the first time is
//! given a new window (front.rs).

mod front;
pub mod ids;
mod select;

pub use front::{front, Front};
pub use select::{on_win, select, select_focused, select_for, select_role, select_window, windows};
