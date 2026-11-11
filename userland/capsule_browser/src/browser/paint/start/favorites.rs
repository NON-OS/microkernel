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

use nonos_app_skeleton::{measure_ttf, PaintBuffer};
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::widgets::text;

use crate::browser::paint::clip;
use crate::browser::paint::start::fav_list::Fav;
use crate::browser::paint::tokens::{self, palette};
use crate::browser::ui::rect::Rect;
use crate::browser::ui::start_geom::{fav_rect, StartGeom};

const TITLE_PX: f32 = 18.0;
const TILE: u32 = 60;
const TILE_RADIUS: u32 = 16;
const LETTER_PX: f32 = 26.0;
const LABEL_PX: f32 = 17.0;
const LABEL_GAP: u32 = 8;
const LABEL_PAD: u32 = 4;

/// The "Favourites" heading and one lettered tile per favourite.
pub fn paint(g: &StartGeom, favs: &[Fav], fb: &mut PaintBuffer) {
    if g.favs == 0 {
        return;
    }
    let first = fav_rect(g, 0);
    let title = scale().font(TITLE_PX);
    fb.text_ttf(first.x, g.fav_title_y, "Favourites", palette().ink2, title);
    for (i, f) in favs.iter().enumerate().take(g.favs) {
        tile(fb, fav_rect(g, i), f);
    }
}

fn tile(fb: &mut PaintBuffer, cell: Rect, f: &Fav) {
    let (s, mut buf) = (scale(), [0u8; 4]);
    let (side, label) = (s.px(TILE) as i32, s.font(LABEL_PX));
    let t = Rect::new(cell.x + (cell.w - side) / 2, cell.y, side, side);
    let (look, radius) = (tokens::category(f.color), s.px(TILE_RADIUS));
    letter_tile(fb, t, radius, f.letter.encode_utf8(&mut buf), look, s.font(LETTER_PX));
    let room = cell.w - 2 * s.px(LABEL_PAD) as i32;
    let shown = text::truncate_by(&f.label, room, |t| measure_ttf(t, label));
    let tx = cell.x + (cell.w - measure_ttf(&shown, label)) / 2;
    let ty = t.bottom() + s.px(LABEL_GAP) as i32;
    fb.text_ttf(tx, ty, &shown, palette().ink2, label);
}

/// A tile in `fill` with `text` centred on it in `ink`, the pair a category
/// gives the ground the tile is drawn on.
pub fn letter_tile(
    fb: &mut PaintBuffer,
    t: Rect,
    radius: u32,
    text: &str,
    (fill, ink): (u32, u32),
    px: f32,
) {
    clip::round(fb, t, radius, fill);
    let lx = t.x + (t.w - measure_ttf(text, px)) / 2;
    let ly = t.y + (t.h - line_height(px)) / 2;
    fb.text_ttf(lx, ly, text, ink, px);
}
