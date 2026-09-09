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

use nonos_app_skeleton::PaintBuffer;

use super::chip::{chip, chip_w, CHIP_H};
use super::fmt_time::fmt_time;
use super::human_size::human_size;
use super::layout::ROW_H;
use super::measure_text::{right_text, truncate_to_width, width_of};
use super::paint_row_tile::row_tile;
use super::recents_group::parent_of;
use super::state::State;
use super::theme::{INK, INK3};

// The trailing meta columns are fixed-width so size and date line up down the
// list; the text inside each is still placed by measurement.
const NAME_PX: f32 = 17.0;
const SUB_PX: f32 = 14.0;
const DATE_W: u32 = 128;
const SIZE_W: u32 = 92;

/// One detail row's content, drawn inside the tile `paint_rows` already laid
/// down. Every column is placed by measurement, never by glyph count.
pub fn row_body(state: &State, fb: &mut PaintBuffer, index: usize, y: u32, left: u32, cw: u32) {
    let entry = &state.entries[index];
    let name_x = row_tile(fb, entry, y, left);
    let date_end = left + cw;
    let size_end = date_end.saturating_sub(DATE_W);
    let tag_end = size_end.saturating_sub(SIZE_W);
    let room = tag_end.saturating_sub(name_x + 16);
    let name = truncate_to_width(fb, entry.label.trim_end_matches('/'), NAME_PX, room);
    let name_w = width_of(fb, name, NAME_PX);
    let _ = fb.text_ttf(name_x as i32, (y + 1) as i32, name, INK, NAME_PX);
    let loc = truncate_to_width(fb, parent_of(&entry.full_path), SUB_PX, name_w);
    let _ = fb.text_ttf(name_x as i32, (y + 22) as i32, loc, INK3, SUB_PX);

    let mut tx = name_x + name_w + 16;
    for tag in state.tags.tags_for(&entry.full_path) {
        if tx + chip_w(tag) > tag_end {
            break;
        }
        tx += chip(fb, tx, y + (ROW_H - CHIP_H) / 2, tag, false);
    }
    if let Some(size) = entry.size {
        right_text(fb, size_end, y + 11, &human_size(size), SUB_PX, INK3);
    }
    if entry.mtime != 0 {
        right_text(fb, date_end, y + 11, &fmt_time(entry.mtime), SUB_PX, INK3);
    }
}
