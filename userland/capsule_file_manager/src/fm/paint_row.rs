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

use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Checkbox, Pill, Rect, Surface};

use super::chip::{chip_gap, chip_w};
use super::layout::{font, metrics, px};
use super::list_cols::{check_w, cols};
use super::list_row_meta::row_meta;
use super::paint_row_tile::row_tile;
use super::recents_group::parent_of;
use super::selection_is_selected::is_selected;
use super::state::State;
use super::target::Target;
use super::target_at::look;

const NAME_PX: f32 = 17.0;
const SUB_PX: f32 = 14.0;
const TAG_GAP: u32 = 16;

/// One detail row's content, drawn over the `Row` `paint_rows` already laid
/// down. Every cell comes from `list_cols::cols` -- the layout the header strip
/// and the hit-test read too -- and every string is placed by measurement,
/// never by glyph count.
pub fn row_body(state: &State, fb: &mut impl Surface, index: usize, y: u32, left: u32, cw: u32) {
    let entry = &state.entries[index];
    let (c, row_h, gap) = (cols(left, cw), metrics().row_h, px(TAG_GAP));
    let (name_px, sub_px) = (font(NAME_PX), font(SUB_PX));
    let on = Checkbox::new(is_selected(state, &entry.full_path));
    let boxed = Rect { x: c.check_x, y, w: check_w(), h: row_h };
    on.state(look(state, Target::Check(index))).draw(fb, boxed);
    let name_x = row_tile(fb, entry, y, c.tile_x);
    let room = c.tag_end.saturating_sub(name_x + gap);
    let name = text::truncate(entry.label.trim_end_matches('/'), room, name_px);
    let (name_w, r) = (text::width(&name, name_px), roles());
    text::left(fb, name_x, (y + px(1)) as i32, &name, r.text, name_px);
    let loc = text::truncate(parent_of(&entry.full_path), name_w, sub_px);
    text::left(fb, name_x, (y + px(22)) as i32, &loc, r.muted, sub_px);

    let mut tx = name_x + name_w + gap;
    for tag in state.tags.tags_for(&entry.full_path) {
        let w = chip_w(tag);
        if tx + w > c.tag_end {
            break;
        }
        Pill::new(tag).draw(fb, Rect { x: tx, y, w, h: row_h });
        tx += w + chip_gap();
    }
    row_meta(fb, entry, y, &c);
}
