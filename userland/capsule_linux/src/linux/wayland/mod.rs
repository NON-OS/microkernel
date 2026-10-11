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
mod bind;
mod buffer;
mod close;
mod commit;
mod configure;
mod data_device;
mod decoration;
mod destructor;
mod display_error;
mod evdev;
mod evdev_keys;
mod exports;
mod fit;
mod focus_step;
mod forget;
mod global;
mod handlers;
mod held_keys;
mod input;
mod object;
mod object_names;
mod ops;
mod keymap;
mod keymap_file;
mod present;
mod route;
mod scene;
mod scene_impl;
mod scene_pixels;
mod seat;
mod frame_len;
mod opaque;
mod pointer_codes;
mod pool_resize;
pub mod window_damage;
mod window_life;
pub mod shm;
mod surface;
mod toplevel;
mod xdg;
mod state;
mod subsurface;
mod out;
mod output;
mod output_enter;
mod popup;
mod region;
mod registry;
mod reshape;
mod serve;
mod unserved;
mod wire;
mod xkb_mods;

pub use exports::*;
