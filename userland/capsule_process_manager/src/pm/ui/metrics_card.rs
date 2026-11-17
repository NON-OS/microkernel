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

//! The card, panel, bar and risk-strip sizes the screens share, in logical
//! pixels, read once at the display scale through `card_metrics()`.

use nonos_toolkit::design::scale::scale;

const CARD_H: u32 = 136;
const CARD_GAP: u32 = 12;
const CARD_PAD: u32 = 13;
const CARD_ICON: u32 = 14;
const CARD_LABEL_GAP: u32 = 8;
const CARD_LINE_GAP: u32 = 5;
const PANEL_PAD: u32 = 15;
const PANEL_HEAD_H: u32 = 34;
const BAR_H: u32 = 7;
const BAR_ROW_H: u32 = 28;
const SPARK_H: u32 = 38;
const RISK_SLOT_W: u32 = 11;
const RISK_SLOT_H: u32 = 14;
const RISK_SLOT_GAP: u32 = 3;

/// The screens' shared sizes at the display scale in force, in drawing pixels.
pub struct CardMetrics {
    pub card_h: u32,
    pub card_gap: u32,
    pub card_pad: u32,
    pub card_icon: u32,
    pub card_label_gap: u32,
    pub card_line_gap: u32,
    pub panel_pad: u32,
    pub panel_head_h: u32,
    pub bar_h: u32,
    pub bar_row_h: u32,
    pub spark_h: u32,
    pub risk_slot_w: u32,
    pub risk_slot_h: u32,
    pub risk_slot_gap: u32,
}

/// The shared sizes at the display scale in force: each logical size above
/// read once through `scale()`, so no caller scales one again.
pub fn card_metrics() -> CardMetrics {
    let s = scale();
    CardMetrics {
        card_h: s.px(CARD_H),
        card_gap: s.px(CARD_GAP),
        card_pad: s.px(CARD_PAD),
        card_icon: s.px(CARD_ICON),
        card_label_gap: s.px(CARD_LABEL_GAP),
        card_line_gap: s.px(CARD_LINE_GAP),
        panel_pad: s.px(PANEL_PAD),
        panel_head_h: s.px(PANEL_HEAD_H),
        bar_h: s.px(BAR_H),
        bar_row_h: s.px(BAR_ROW_H),
        spark_h: s.px(SPARK_H),
        risk_slot_w: s.px(RISK_SLOT_W),
        risk_slot_h: s.px(RISK_SLOT_H),
        risk_slot_gap: s.px(RISK_SLOT_GAP),
    }
}
