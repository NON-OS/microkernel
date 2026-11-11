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

// Every layout size the restyle uses, in logical pixels: `metrics()` and
// `card_metrics()` read each once at the display scale. Nothing here is sized
// below ttf::MIN_UI_PX (17.0), which clamps both draw and measure. The window's
// opening size stays logical, as the manifest declares it, and so do the row
// height and column widths, which the shared table scales itself.

use nonos_toolkit::design::scale::scale;

#[path = "metrics_card.rs"]
mod card;
pub use card::card_metrics;

pub const WIN_W: u32 = 1240;
pub const WIN_H: u32 = 780;
const SIDEBAR_W: u32 = 196;
const NAV_TOP: u32 = 14;
const NAV_H: u32 = 36;
const NAV_GAP: u32 = 2;
const NAV_PAD_X: u32 = 12;
const SIDE_FOOT_H: u32 = 54;
const SIDE_FOOT_TOP: u32 = 10;
const SIDE_FOOT_LINE: u32 = 22;

const PANE_PAD_X: u32 = 20;
const PANE_PAD_TOP: u32 = 16;
const HEAD_H: u32 = 46;
const STATUS_H: u32 = 26;
const STATUS_GAP: u32 = 8;
const STATUS_GROUP_GAP: u32 = 22;
const SEARCH_W: u32 = 240;
const SEARCH_MIN_W: u32 = 168;
const SEARCH_H: u32 = 30;
const SEARCH_META_GAP: u32 = 14;
const INSPECTOR_W: u32 = 280;
const INSPECTOR_PAD: u32 = 15;
const INSP_FIELD_H: u32 = 25;
const INSP_SECTION_GAP: u32 = 14;
const INSP_BTN_H: u32 = 34;
const INSP_BTN_BOTTOM: u32 = 40;

const CHIP_H: u32 = 30;
const CHIP_PAD_X: u32 = 11;
const CHIP_GAP: u32 = 8;
/// The shared table's row and header height, which its geometry scales.
pub const ROW_H: u32 = nonos_toolkit::design::size::TABLE_ROW_H;
pub const COL_PID_W: u32 = 56;
pub const COL_STATE_W: u32 = 84;
pub const COL_CPU_W: u32 = 84;
pub const COL_MEM_W: u32 = 88;
pub const COL_AUTH_W: u32 = 100;
pub const COL_RATE_W: u32 = 74;

const TITLE_PX: f32 = 21.0;
const CARD_VALUE_PX: f32 = 27.0;
const BODY_PX: f32 = 17.0;
const NUM_PX: f32 = 17.0;

/// The window's frame sizes at the display scale in force, in drawing pixels.
pub struct Metrics {
    pub sidebar_w: u32,
    pub nav_top: u32,
    pub nav_h: u32,
    pub nav_gap: u32,
    pub nav_pad_x: u32,
    pub side_foot_h: u32,
    pub side_foot_top: u32,
    pub side_foot_line: u32,
    pub pane_pad_x: u32,
    pub pane_pad_top: u32,
    pub head_h: u32,
    pub status_h: u32,
    pub status_gap: u32,
    pub status_group_gap: u32,
    pub search_w: u32,
    pub search_min_w: u32,
    pub search_h: u32,
    pub search_meta_gap: u32,
    pub inspector_w: u32,
    pub inspector_pad: u32,
    pub insp_field_h: u32,
    pub insp_section_gap: u32,
    pub insp_btn_h: u32,
    pub insp_btn_bottom: u32,
    pub chip_h: u32,
    pub chip_pad_x: u32,
    pub chip_gap: u32,
    pub title_px: f32,
    pub card_value_px: f32,
    pub body_px: f32,
    pub num_px: f32,
}

/// The frame sizes at the display scale in force: each logical size above
/// read once through `scale()`, so no caller scales one again.
pub fn metrics() -> Metrics {
    let s = scale();
    Metrics {
        sidebar_w: s.px(SIDEBAR_W),
        nav_top: s.px(NAV_TOP),
        nav_h: s.px(NAV_H),
        nav_gap: s.px(NAV_GAP),
        nav_pad_x: s.px(NAV_PAD_X),
        side_foot_h: s.px(SIDE_FOOT_H),
        side_foot_top: s.px(SIDE_FOOT_TOP),
        side_foot_line: s.px(SIDE_FOOT_LINE),
        pane_pad_x: s.px(PANE_PAD_X),
        pane_pad_top: s.px(PANE_PAD_TOP),
        head_h: s.px(HEAD_H),
        status_h: s.px(STATUS_H),
        status_gap: s.px(STATUS_GAP),
        status_group_gap: s.px(STATUS_GROUP_GAP),
        search_w: s.px(SEARCH_W),
        search_min_w: s.px(SEARCH_MIN_W),
        search_h: s.px(SEARCH_H),
        search_meta_gap: s.px(SEARCH_META_GAP),
        inspector_w: s.px(INSPECTOR_W),
        inspector_pad: s.px(INSPECTOR_PAD),
        insp_field_h: s.px(INSP_FIELD_H),
        insp_section_gap: s.px(INSP_SECTION_GAP),
        insp_btn_h: s.px(INSP_BTN_H),
        insp_btn_bottom: s.px(INSP_BTN_BOTTOM),
        chip_h: s.px(CHIP_H),
        chip_pad_x: s.px(CHIP_PAD_X),
        chip_gap: s.px(CHIP_GAP),
        title_px: s.font(TITLE_PX),
        card_value_px: s.font(CARD_VALUE_PX),
        body_px: s.font(BODY_PX),
        num_px: s.font(NUM_PX),
    }
}
