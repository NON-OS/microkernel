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
use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::theme::Category;
use nonos_toolkit::widgets::{Ink, Label};

use crate::browser::paint::pages::cells::px;
use crate::browser::paint::start::{letter_tile, tile_letter};
use crate::browser::ui::grid_geom::close_rect;
use crate::browser::ui::rect::{area, Rect};

pub const PAD: u32 = 14;
pub const HEAD_H: u32 = 64;
const TILE: u32 = 32;
const TILE_RADIUS: u32 = 9;
const TEXT_GAP: u32 = 10;

/// The card's top strip: the site tile, then title over host up to the
/// close button. Drawn only on a card whose top is on the page.
pub fn head(fb: &mut PaintBuffer, r: Rect, title: &str, host: &str, k: Category) {
    if r.y < 0 {
        return;
    }
    let (head, side) = (px(HEAD_H), px(TILE));
    let mut buf = [0u8; 4];
    let letter = tile_letter(host).encode_utf8(&mut buf);
    let t = Rect::new(r.x + px(PAD), r.y + (head - side) / 2, side, side);
    let body = scale().font(TextRole::Body.px());
    letter_tile(fb, t, scale().px(TILE_RADIUS), letter, k, body);
    let x = t.right() + px(TEXT_GAP);
    let (w, lh) = (close_rect(r).x - x, line_height(body));
    let top = r.y + (head - 2 * lh) / 2;
    Label::new(title).draw(fb, area(Rect::new(x, top, w, lh)));
    Label::new(host).ink(Ink::Muted).draw(fb, area(Rect::new(x, top + lh, w, lh)));
}
