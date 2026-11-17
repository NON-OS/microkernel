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

// Every layout size the rebuild uses, in logical pixels. No type size here
// sits below ttf::MIN_UI_PX (17.0), which clamps both draw and measure.

use nonos_toolkit::design::scale::scale;

const PAD: u32 = 20;
const PAD_TIGHT: u32 = 12;
const GAP: u32 = 14;
const GAP_TIGHT: u32 = 8;
const GAP_WIDE: u32 = 22;

const RADIUS_PANEL: u32 = 14;
const RADIUS_CELL: u32 = 4;

const RAIL_W: u32 = 292;
const HUD_H: u32 = 92;
const HUD_CARD_GAP: u32 = 12;
const FOOT_H: u32 = 56;
const FOOT_BTN_W: u32 = 168;
const FOOT_BTN_H: u32 = 38;

const BTN_H: u32 = 44;
const BTN_W: u32 = 268;
const BTN_GAP: u32 = 10;
const CHIP_H: u32 = 34;
const CHIP_GAP: u32 = 9;

const MODAL_W: u32 = 420;
const MODAL_H: u32 = 336;
const ROW_H: u32 = 34;
const TABLE_HEAD_H: u32 = 30;
pub const RANK_ROWS: usize = 10;

const ICON_SM: u32 = 14;
const ICON_MD: u32 = 18;

const PX_WORDMARK: f32 = 52.0;
const PX_TITLE: f32 = 30.0;
const PX_HEAD: f32 = 22.0;
const PX_BODY: f32 = 18.0;
const PX_LABEL: f32 = 17.0;
const PX_STAT: f32 = 27.0;

/// Every layout size at the display scale in force, in drawing pixels.
pub struct Metrics {
    pub pad: u32,
    pub pad_tight: u32,
    pub gap: u32,
    pub gap_tight: u32,
    pub gap_wide: u32,
    pub radius_panel: u32,
    pub radius_cell: u32,
    pub rail_w: u32,
    pub hud_h: u32,
    pub hud_card_gap: u32,
    pub foot_h: u32,
    pub foot_btn_w: u32,
    pub foot_btn_h: u32,
    pub btn_h: u32,
    pub btn_w: u32,
    pub btn_gap: u32,
    pub chip_h: u32,
    pub chip_gap: u32,
    pub modal_w: u32,
    pub modal_h: u32,
    pub row_h: u32,
    pub table_head_h: u32,
    pub icon_sm: u32,
    pub icon_md: u32,
    pub px_wordmark: f32,
    pub px_title: f32,
    pub px_head: f32,
    pub px_body: f32,
    pub px_label: f32,
    pub px_stat: f32,
}

/// The layout sizes at the display scale in force: each logical size above
/// read once through `scale()`.
pub fn metrics() -> Metrics {
    let s = scale();
    Metrics {
        pad: s.px(PAD),
        pad_tight: s.px(PAD_TIGHT),
        gap: s.px(GAP),
        gap_tight: s.px(GAP_TIGHT),
        gap_wide: s.px(GAP_WIDE),
        radius_panel: s.px(RADIUS_PANEL),
        radius_cell: s.px(RADIUS_CELL),
        rail_w: s.px(RAIL_W),
        hud_h: s.px(HUD_H),
        hud_card_gap: s.px(HUD_CARD_GAP),
        foot_h: s.px(FOOT_H),
        foot_btn_w: s.px(FOOT_BTN_W),
        foot_btn_h: s.px(FOOT_BTN_H),
        btn_h: s.px(BTN_H),
        btn_w: s.px(BTN_W),
        btn_gap: s.px(BTN_GAP),
        chip_h: s.px(CHIP_H),
        chip_gap: s.px(CHIP_GAP),
        modal_w: s.px(MODAL_W),
        modal_h: s.px(MODAL_H),
        row_h: s.px(ROW_H),
        table_head_h: s.px(TABLE_HEAD_H),
        icon_sm: s.px(ICON_SM),
        icon_md: s.px(ICON_MD),
        px_wordmark: s.font(PX_WORDMARK),
        px_title: s.font(PX_TITLE),
        px_head: s.font(PX_HEAD),
        px_body: s.font(PX_BODY),
        px_label: s.font(PX_LABEL),
        px_stat: s.font(PX_STAT),
    }
}
