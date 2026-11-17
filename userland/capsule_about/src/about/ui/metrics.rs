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

// Every layout size the restyle uses, in logical pixels: `metrics()` reads
// each once at the display scale. Nothing here is sized below ttf::MIN_UI_PX
// (17.0), which clamps both draw and measure. The window's own opening size
// and place stay logical, as the manifest declares them.

use nonos_toolkit::design::scale::scale;

pub const WIN_W: u32 = 1000;
pub const WIN_H: u32 = 680;
pub const WIN_X: u32 = 188;
pub const WIN_Y: u32 = 52;

const SIDEBAR_W: u32 = 196;
const NAV_TOP: u32 = 14;
const NAV_H: u32 = 36;
const NAV_GAP: u32 = 2;
const NAV_PAD_X: u32 = 12;

const PANE_PAD_X: u32 = 20;
const PANE_PAD_TOP: u32 = 16;
const HEAD_H: u32 = 46;
const HEAD_META_GAP: u32 = 18;
const STATUS_H: u32 = 26;

const CARD_PAD: u32 = 16;
const CARD_GAP: u32 = 14;
// The air a list card keeps under its last row.
const CARD_FOOT: u32 = 4;

const KV_ROW_H: u32 = 26;
const KV_LABEL_W: u32 = 172;
const KV_GAP: u32 = 12;

const CHIP_GAP: u32 = 8;

// One arrow key; a wheel notch is three (scroll_wheel.rs). Scroll is in
// pixels, so a step is a size, not a row count: see state.rs.
const SCROLL_STEP: u32 = 24;

const TITLE_PX: f32 = 21.0;
const VALUE_PX: f32 = 27.0;
const BODY_PX: f32 = 17.0;
const NUM_PX: f32 = 17.0;

// Blocks the screens lay out with, in the same space as the card and row
// metrics above.
const HERO_H: u32 = 124;
const PAIR_H: u32 = 50;
const PAIR_LINE: u32 = 24;
const CHAIN_H: u32 = 30;
const LINE_STEP: u32 = 24;

/// Every shared layout size at the display scale in force, in drawing pixels.
pub struct Metrics {
    pub sidebar_w: u32,
    pub nav_top: u32,
    pub nav_h: u32,
    pub nav_gap: u32,
    pub nav_pad_x: u32,
    pub pane_pad_x: u32,
    pub pane_pad_top: u32,
    pub head_h: u32,
    pub head_meta_gap: u32,
    pub status_h: u32,
    pub card_pad: u32,
    pub card_gap: u32,
    pub card_foot: u32,
    pub kv_row_h: u32,
    pub kv_label_w: u32,
    pub kv_gap: u32,
    pub chip_gap: u32,
    pub scroll_step: u32,
    pub hero_h: u32,
    pub pair_h: u32,
    pub pair_line: u32,
    pub chain_h: u32,
    pub line_step: u32,
    pub title_px: f32,
    pub value_px: f32,
    pub body_px: f32,
    pub num_px: f32,
}

/// The shared sizes at the display scale in force: each logical size above
/// read once through `scale()`, so no caller scales one again.
pub fn metrics() -> Metrics {
    let s = scale();
    Metrics {
        sidebar_w: s.px(SIDEBAR_W),
        nav_top: s.px(NAV_TOP),
        nav_h: s.px(NAV_H),
        nav_gap: s.px(NAV_GAP),
        nav_pad_x: s.px(NAV_PAD_X),
        pane_pad_x: s.px(PANE_PAD_X),
        pane_pad_top: s.px(PANE_PAD_TOP),
        head_h: s.px(HEAD_H),
        head_meta_gap: s.px(HEAD_META_GAP),
        status_h: s.px(STATUS_H),
        card_pad: s.px(CARD_PAD),
        card_gap: s.px(CARD_GAP),
        card_foot: s.px(CARD_FOOT),
        kv_row_h: s.px(KV_ROW_H),
        kv_label_w: s.px(KV_LABEL_W),
        kv_gap: s.px(KV_GAP),
        chip_gap: s.px(CHIP_GAP),
        scroll_step: s.px(SCROLL_STEP),
        hero_h: s.px(HERO_H),
        pair_h: s.px(PAIR_H),
        pair_line: s.px(PAIR_LINE),
        chain_h: s.px(CHAIN_H),
        line_step: s.px(LINE_STEP),
        title_px: s.font(TITLE_PX),
        value_px: s.font(VALUE_PX),
        body_px: s.font(BODY_PX),
        num_px: s.font(NUM_PX),
    }
}
