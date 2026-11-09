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

use alloc::vec::Vec;

use super::list_geom::ListGeom;
use super::rect::Rect;
use super::settings_cells::px;
use super::settings_place::{place, Cursor};
pub use super::settings_rows::{Control, Field, Item, Kind, Line};
use super::settings_table::SECTIONS;

pub const BAND_H: i32 = 112;
const HEAD_H: u32 = 44;
const CARD_PAD: u32 = 6;
const SECTION_GAP: u32 = 20;
const BOTTOM: i32 = 32;
const MAX_W: u32 = 960;
const SIDE: u32 = 40;

/// Where everything on the settings page sits, in content-local pixels.
/// The header band stays put; the sections under it move up by `scroll`.
/// The column is the list pages' at 1x and widens with the display scale.
/// The painter and the click handler build it the same way.
pub struct SettingsGeom {
    pub head: Rect,
    pub items: Vec<Item>,
    pub height: i32,
}

impl SettingsGeom {
    pub fn new(w: i32, h: i32, scroll: i32) -> Self {
        let list = ListGeom::new(w, h, 0);
        let fw = px(MAX_W).min((w - 2 * px(SIDE)).max(0));
        let frame = Rect::new((w - fw) / 2, 0, fw, h);
        let mut items = Vec::new();
        let (head_h, card_pad, section_gap) = (px(HEAD_H), px(CARD_PAD), px(SECTION_GAP));
        let top = BAND_H - scroll.max(0);
        let mut y = top;
        for (title, rows) in SECTIONS.iter() {
            items.push(Item {
                kind: Kind::Head(title),
                rect: Rect::new(frame.x, y, frame.w, head_h),
            });
            y += head_h;
            let card = items.len();
            items.push(Item { kind: Kind::Card, rect: Rect::default() });
            let mut at = Cursor { frame, y: y + card_pad };
            for (i, row) in rows.iter().enumerate() {
                place(&mut at, row, i > 0, &mut items);
            }
            items[card].rect = Rect::new(frame.x, y, frame.w, at.y + card_pad - y);
            y = at.y + card_pad + section_gap;
        }
        let height = y - top + BAND_H + BOTTOM - section_gap;
        SettingsGeom { head: list.head, items, height }
    }

    /// Where `c` is drawn, if the page has it.
    pub fn rect(&self, c: Control) -> Option<Rect> {
        self.items.iter().find(|it| it.kind == Kind::Ctl(c)).map(|it| it.rect)
    }
}
