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

//! The pane's items the cursor never stops on: the page head, the cards and
//! their heads, and the Wi-Fi page's network and saved rows.

use nonos_app_skeleton::PaintBuffer;

use crate::settings::schema::blocks_for;
use crate::settings::state::State;

use super::walk::Item;
use super::{card, net_rows, page_head, row, saved_rows};

/// `item` at screen row `sy`, `h` tall, in a card from `card_x`, `card_w` wide.
pub fn paint(
    fb: &mut PaintBuffer,
    state: &State,
    item: Item,
    card: (u32, u32),
    at: (i32, u32),
    view_h: u32,
) {
    let ((card_x, card_w), (sy, h)) = (card, at);
    let blocks = blocks_for(state.section);
    match item {
        Item::Head => page_head::paint(fb, state.section, sy),
        Item::Card => card::paint_body(fb, card_x, sy, card_w, h, view_h),
        Item::CardHead(bi) => card::paint_head(fb, state, &blocks[bi], card_x, sy, card_w),
        Item::Network(i) => {
            net_rows::paint(fb, state, i, card_x, card_w, sy, h);
            row::hairline(fb, card_x, card_w, sy, h);
        }
        Item::Saved(i) => {
            saved_rows::paint(fb, state, i, card_x, card_w, sy, h);
            row::hairline(fb, card_x, card_w, sy, h);
        }
        Item::Row(..) | Item::Wallpaper(_) => {}
    }
}
