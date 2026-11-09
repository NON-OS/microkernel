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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::theme::Category;
use nonos_toolkit::widgets::Rect as Area;

use crate::browser::paint::start::{letter_tile, tile_color, tile_letter};
use crate::browser::ui::rect::Rect;

pub const TILE: u32 = 40;
pub const WIDE_TILE: u32 = 52;
const TILE_RADIUS: u32 = 10;

/// `n` logical pixels at the display scale.
pub fn px(n: u32) -> i32 {
    scale().px(n) as i32
}

/// A tile `w` wide and `TILE` tall at the left of `lead`, centred down
/// it, with `text` on it.
pub fn tile(fb: &mut PaintBuffer, lead: Area, text: &str, k: Category, w: i32) {
    let (l, side) = (back(lead), px(TILE));
    let t = Rect::new(l.x, l.y + (l.h - side) / 2, w, side);
    let body = scale().font(TextRole::Body.px());
    letter_tile(fb, t, scale().px(TILE_RADIUS), text, k, body);
}

/// The lettered tile the start page gives `host`, in the box `lead`.
pub fn site_tile(fb: &mut PaintBuffer, lead: Area, host: &str) {
    let mut buf = [0u8; 4];
    let letter = tile_letter(host).encode_utf8(&mut buf);
    tile(fb, lead, letter, tile_color(host), px(TILE));
}

/// `r` as the toolkit's rect, its negative parts cut off.
pub fn area(r: Rect) -> Area {
    let (x, y) = (r.x.max(0), r.y.max(0));
    let (w, h) = ((r.right() - x).max(0), (r.bottom() - y).max(0));
    Area { x: x as u32, y: y as u32, w: w as u32, h: h as u32 }
}

/// The toolkit's rect `a` as the browser's.
pub fn back(a: Area) -> Rect {
    Rect::new(a.x as i32, a.y as i32, a.w as i32, a.h as i32)
}

/// The part of `r` below `band`, which alone takes clicks.
pub fn below(r: Rect, band: i32) -> Rect {
    let top = r.y.max(band);
    Rect::new(r.x, top, r.w, (r.bottom() - top).max(0))
}
