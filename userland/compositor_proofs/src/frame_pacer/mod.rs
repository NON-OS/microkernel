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

//! The frame pacer's pure parts: composing a damaged rectangle onto real
//! pixels with the arrow cursor drawn last, a screenshot's capture, the band,
//! the looks, and how damage is drained in batches.

#[path = "../../../compositor/src/frame_pacer/band_rule.rs"]
pub mod band_rule;
#[path = "../../../compositor/src/frame_pacer/canvas_copy.rs"]
pub mod canvas_copy;
#[path = "../../../compositor/src/frame_pacer/compose.rs"]
pub mod compose;
#[path = "../../../compositor/src/frame_pacer/cursor.rs"]
pub mod cursor;
#[path = "../../../compositor/src/frame_pacer/cursor_shape.rs"]
pub mod cursor_shape;
#[path = "../../../compositor/src/frame_pacer/drain_batch.rs"]
pub mod drain_batch;
#[path = "../../../compositor/src/frame_pacer/drain_damage.rs"]
pub mod drain_damage;
#[path = "../../../compositor/src/frame_pacer/band_glyph_art.rs"]
pub mod band_glyph_art;
#[path = "../../../compositor/src/frame_pacer/band_glyphs.rs"]
pub mod band_glyphs;
#[path = "../../../compositor/src/frame_pacer/band_meet.rs"]
pub mod band_meet;
#[path = "../../../compositor/src/frame_pacer/indicator_band.rs"]
pub mod indicator_band;
#[path = "../../../compositor/src/frame_pacer/look.rs"]
pub mod look;
#[path = "../../../compositor/src/frame_pacer/look_matrix.rs"]
pub mod look_matrix;
#[path = "../../../compositor/src/frame_pacer/night_tint.rs"]
pub mod night_tint;
#[path = "../../../compositor/src/frame_pacer/own_look.rs"]
pub mod own_look;
#[path = "../../../compositor/src/frame_pacer/present_note.rs"]
pub mod present_note;
