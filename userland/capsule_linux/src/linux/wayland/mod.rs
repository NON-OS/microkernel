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


//! The display server a Wayland client finds when it connects.

mod args;
mod buffer;
mod close_window;
mod commit;
mod configure;
mod fit;
mod forget;
mod handlers;
mod input;
mod input_enter;
mod input_key;
mod input_send;
mod object;
mod ops;
mod keymap;
mod keymap_file;
mod scene;
mod scene_pixels;
mod seat;
mod frame_len;
mod present;
mod present_surface;
mod present_window;
pub mod window_damage;
mod window_life;
pub mod shm;
mod surface;
mod toplevel;
mod xdg;
mod state;
mod out;
mod output;
mod registry;
mod reshape;
mod route;
mod serve;
mod unserved;
mod wire;

pub use close_window::{close_window, disconnect};
pub use object::Objects;
pub use window_life::ends_connection;
pub use scene::Scene;
pub use input::pump;
pub use serve::serve;
