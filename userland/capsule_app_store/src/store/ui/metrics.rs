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
// Every layout size in logical pixels. The list is a column of cards
/*
 * rather than table rows: a row of one name reads as a database dump, and the
 * catalogue has a description and a publisher for every entry that were going
 * unshown.
 */

use nonos_toolkit::design::scale::scale;

pub const WIN_W: u32 = 1000;
pub const WIN_H: u32 = 680;
pub const WIN_X: u32 = 188;
pub const WIN_Y: u32 = 52;

const PAD_X: u32 = 22;
const PAD_TOP: u32 = 18;
const HEAD_H: u32 = 44;
const TAB_H: u32 = 32;
/// Air between the tab strip and the first card.
const TAB_TO_LIST: u32 = 10;

/// The search field in the head band, right-aligned.
const SEARCH_W: u32 = 260;
const SEARCH_H: u32 = 26;

/// A card holds two lines of text over a tile, so it is tall enough for
/// both plus the breathing room that stops a list looking like a table.
const CARD_H: u32 = 64;
const CARD_GAP: u32 = 6;
const CARD_PAD: u32 = 14;
const TILE: u32 = 36;
const TILE_GAP: u32 = 14;
/// The scrollbar beside the list, this far right of the cards.
const SCROLL_GAP: u32 = 6;
const SCROLL_W: u32 = 3;

/// The action sits at a fixed width on the right so every card's button
/// starts at the same x and the eye can run straight down them.
const ACTION_W: u32 = 96;
const ACTION_H: u32 = 28;

const DETAIL_W: u32 = 332;
const DETAIL_PAD: u32 = 18;
const GATE_ROW_H: u32 = 24;

const STATUS_H: u32 = 28;
const STATUS_PAD_X: u32 = 16;

const TITLE_PX: f32 = 23.0;
const NAME_PX: f32 = 18.0;
const BODY_PX: f32 = 17.0;
const SMALL_PX: f32 = 17.0;

/// Every layout size at the display scale in force, in drawing pixels.
pub struct Metrics {
    pub pad_x: u32,
    pub pad_top: u32,
    pub head_h: u32,
    pub tab_h: u32,
    pub tab_to_list: u32,
    pub search_w: u32,
    pub search_h: u32,
    pub card_h: u32,
    pub card_gap: u32,
    pub card_pad: u32,
    pub tile: u32,
    pub tile_gap: u32,
    pub scroll_gap: u32,
    pub scroll_w: u32,
    pub action_w: u32,
    pub action_h: u32,
    pub detail_w: u32,
    pub detail_pad: u32,
    pub gate_row_h: u32,
    pub status_h: u32,
    pub status_pad_x: u32,
    pub title_px: f32,
    pub name_px: f32,
    pub body_px: f32,
    pub small_px: f32,
}

/// The layout sizes at the display scale in force: each logical size above
/// read once through `scale()`.
pub fn metrics() -> Metrics {
    let s = scale();
    Metrics {
        pad_x: s.px(PAD_X),
        pad_top: s.px(PAD_TOP),
        head_h: s.px(HEAD_H),
        tab_h: s.px(TAB_H),
        tab_to_list: s.px(TAB_TO_LIST),
        search_w: s.px(SEARCH_W),
        search_h: s.px(SEARCH_H),
        card_h: s.px(CARD_H),
        card_gap: s.px(CARD_GAP),
        card_pad: s.px(CARD_PAD),
        tile: s.px(TILE),
        tile_gap: s.px(TILE_GAP),
        scroll_gap: s.px(SCROLL_GAP),
        scroll_w: s.px(SCROLL_W),
        action_w: s.px(ACTION_W),
        action_h: s.px(ACTION_H),
        detail_w: s.px(DETAIL_W),
        detail_pad: s.px(DETAIL_PAD),
        gate_row_h: s.px(GATE_ROW_H),
        status_h: s.px(STATUS_H),
        status_pad_x: s.px(STATUS_PAD_X),
        title_px: s.font(TITLE_PX),
        name_px: s.font(NAME_PX),
        body_px: s.font(BODY_PX),
        small_px: s.font(SMALL_PX),
    }
}
