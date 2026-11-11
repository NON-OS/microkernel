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

use super::rect::Rect;
use super::settings_cells::px;
use super::start_cells::{cards, cell};
pub use super::start_cells::{fav_at, fav_rect, pill_text};

pub const MAX_FAVS: usize = 8;
const HERO_MIN: u32 = 220;
const PILL_W: u32 = 640;
const PILL_H: u32 = 52;
const PILL_LIFT: u32 = 32;
const PILL_ROOM: u32 = 80;
const SIDE: u32 = 32;
const BOTTOM: u32 = 32;
const TITLE_GAP: u32 = 24;
const TILES_GAP: u32 = 36;
const CARDS_GAP: u32 = 24;
const GREET_UP: u32 = 124;
const SUB_UP: u32 = 64;

/// Where everything on the start page sits, in content-local pixels. The
/// painter and the click handler both build it from the same inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartGeom {
    pub hero: Rect,
    pub greet_y: i32,
    pub sub_y: i32,
    pub pill: Rect,
    pub fav_title_y: i32,
    pub tiles_y: i32,
    pub favs: usize,
    pub cols: i32,
    pub cards: [Rect; 3],
}

pub fn compute(w: i32, h: i32, favs: usize, scroll: i32) -> StartGeom {
    let (w, h, favs) = (w.max(0), h.max(0), favs.min(MAX_FAVS));
    let hero = Rect::new(0, -scroll, w, (h * 36 / 100).max(px(HERO_MIN)));
    let (pw, ph, (cell_w, cell_h)) =
        (px(PILL_W).min((w - px(PILL_ROOM)).max(0)), px(PILL_H), cell());
    let pill = Rect::new((w - pw) / 2, hero.bottom() - ph - px(PILL_LIFT), pw, ph);
    let cols = (favs as i32).min(((w - 2 * px(SIDE)) / cell_w).max(1)).max(1);
    let fav_title_y = hero.bottom() + px(TITLE_GAP);
    let tiles_y = fav_title_y + px(TILES_GAP);
    let fav_rows = (favs as i32 + cols - 1) / cols;
    let cards = cards(w, tiles_y + fav_rows * cell_h + px(CARDS_GAP));
    StartGeom {
        hero,
        greet_y: pill.y - px(GREET_UP),
        sub_y: pill.y - px(SUB_UP),
        pill,
        fav_title_y,
        tiles_y,
        favs,
        cols,
        cards,
    }
}

/// The page scrolled by `scroll`, held between its top and the point where
/// its last card reaches the bottom of an `h` tall view.
pub fn scrolled(w: i32, h: i32, favs: usize, scroll: i32) -> StartGeom {
    let full = compute(w, h, favs, 0);
    let most = (full.cards[2].bottom() + px(BOTTOM) - h.max(0)).max(0);
    compute(w, h, favs, scroll.clamp(0, most))
}
