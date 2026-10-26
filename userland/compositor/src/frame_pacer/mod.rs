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

pub mod band_glyph_art;
pub mod band_glyphs;
pub mod band_meet;
pub mod band_rule;
pub mod band_watch;
pub mod canvas_copy;
pub mod clip;
pub mod compose;
pub mod composite;
pub mod cursor;
pub mod cursor_arrow;
pub mod cursor_diagonals;
pub mod cursor_shape;
pub mod cursor_shapes;
pub mod cursor_slot;
pub mod cursor_straights;
pub mod drain_batch;
pub mod drain_damage;
pub mod extend;
pub mod first_frame;
pub mod hw_cursor;
pub mod hw_cursor_back;
pub mod hw_cursor_send;
pub mod indicator_band;
pub mod lens;
pub mod look;
pub mod look_matrix;
pub mod look_paint;
pub mod mirror;
pub mod night_tint;
pub mod own_look;
pub mod present_note;
pub mod present_wire;
pub mod show;
pub mod show_band;
pub mod slice_view;
pub mod tick;
pub mod to_screen;

pub use show_band::show_band;
pub use tick::tick;
