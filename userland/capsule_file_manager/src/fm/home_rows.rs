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
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::{roles, Opaque};
use nonos_toolkit::widgets::{draw as paint, text, Rect, Row, Separator, State as Look};

use super::file_color::color;
use super::file_kind::kind_of_name;
use super::layout::{font, hair, px};
use super::recents_group::parent_of;
use super::sidebar_rows::base_label;

/// Home stacks its journal rows tighter than the full-screen surfaces do, so the
/// category and storage rows both keep their band at the default window size.
pub const HOME_ROW_H: u32 = 44;

const ROW_ICON: u32 = 20;
const ICON_GAP: u32 = 14;
const META_GAP: u32 = 12;
const TITLE_PX: f32 = 15.0;
const SUB_PX: f32 = 13.0;

/// One Continue Working row: the entry, the directory it came out of, and how
/// long ago it was touched. The title column is whatever the measured meta
/// column leaves, so a long name is cut on a char boundary instead of running
/// under the timestamp. The row is a shared `Row` body drawn in `look`.
pub fn recent_row(fb: &mut PaintBuffer, at: Rect, path: &str, meta: &str, look: Look) {
    let Rect { x, y, w, h } = at;
    let (icon, title_px, sub_px) = (px(ROW_ICON), font(TITLE_PX), font(SUB_PX));
    Row::new("").state(look).draw(fb, at);
    let dir = path.ends_with('/');
    let r = roles();
    let tint = if dir { r.accent } else { Opaque::new(color(kind_of_name(path))) };
    let glyph = if dir { IconId::FsFolder } else { IconId::FsFile };
    paint::icon(fb, glyph, x, y + (h - icon) / 2, icon, tint);
    let tx = x + icon + px(ICON_GAP);
    let room = (x + w).saturating_sub(tx + text::width(meta, sub_px) + px(META_GAP));
    let title = base_label(path);
    let cut = text::truncate(title.as_str(), room, title_px);
    text::left(fb, tx, (y + px(1)) as i32, &cut, r.text, title_px);
    let sub = text::truncate(parent_of(path), room, sub_px);
    text::left(fb, tx, (y + px(22)) as i32, &sub, r.muted, sub_px);
    text::right(fb, x + w, (y + px(1)) as i32, meta, r.muted, sub_px);
    let line = hair();
    Separator::new().draw(fb, Rect { x, y: y + h - line, w, h: line });
}
