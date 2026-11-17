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

use nonos_toolkit::design::scale::scale;

const RAIL_W: i32 = 212;
const RAIL_PAD_X: i32 = 12;
const BRAND_TOP: i32 = 16;
const BRAND_H: i32 = 48;
const NAV_TOP: i32 = 76;
const NAV_H: i32 = 44;
const NAV_GAP: i32 = 4;

const PANE_PAD: i32 = 18;
const READOUT_H: i32 = 112;
const KEY_GAP: i32 = 10;
const R_KEY: i32 = 9;

/// The readout's largest size, the head of its fit ladder, in logical pixels.
pub const PX_RESULT: f32 = 56.0;
const PX_TITLE: f32 = 17.0;
const PX_BODY: f32 = 17.0;
const PX_MONO: f32 = 17.0;

/// `logical` pixels at the display scale in force, in drawing pixels.
pub fn px(logical: i32) -> i32 {
    scale().px(logical.max(0) as u32) as i32
}

/// Every shared layout size at the display scale in force, in drawing pixels.
pub struct Metrics {
    pub rail_w: i32,
    pub rail_pad_x: i32,
    pub rail_x: i32,
    pub rail_card_w: i32,
    pub brand_top: i32,
    pub brand_h: i32,
    pub nav_top: i32,
    pub nav_h: i32,
    pub nav_gap: i32,
    pub pane_pad: i32,
    pub readout_h: i32,
    pub key_gap: i32,
    pub r_key: i32,
    pub px_title: f32,
    pub px_body: f32,
    pub px_mono: f32,
}

/// The layout sizes at the display scale in force: each logical size above
/// read once through `scale()`. The rail card ends a pane gutter short of
/// the rail, so it is cut from the scaled sizes and the gutter stays whole.
pub fn metrics() -> Metrics {
    let (s, pane_pad, rail_w) = (scale(), px(PANE_PAD), px(RAIL_W));
    Metrics {
        rail_w,
        rail_pad_x: px(RAIL_PAD_X),
        rail_x: pane_pad,
        rail_card_w: rail_w - pane_pad,
        brand_top: px(BRAND_TOP),
        brand_h: px(BRAND_H),
        nav_top: px(NAV_TOP),
        nav_h: px(NAV_H),
        nav_gap: px(NAV_GAP),
        pane_pad,
        readout_h: px(READOUT_H),
        key_gap: px(KEY_GAP),
        r_key: px(R_KEY),
        px_title: s.font(PX_TITLE),
        px_body: s.font(PX_BODY),
        px_mono: s.font(PX_MONO),
    }
}
