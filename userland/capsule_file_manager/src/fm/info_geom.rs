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

use super::info_sizes::{body_h, PAD, QUICK_H, SEC_GAP, SEC_H};
use super::info_tag_add::band_h;
use super::layout::{info_x, metrics, px};
use super::state::State;

const FOOT_PAD: u32 = 20;

/// Where every band of the info panel starts. One pass, consumed by the painter
/// and by the hit-test, so a control can never be drawn somewhere a click is not
/// looked for. Nothing here measures a glyph off a surface: the widths it needs
/// come from the free `measure_ttf` facade underneath `chip_w`.
pub struct InfoGeom {
    pub x: u32,
    pub w: u32,
    pub title_y: u32,
    pub loc_y: u32,
    pub rule_y: u32,
    pub quick_head: u32,
    pub quick_y: u32,
    pub body_top: u32,
    pub tags_head: u32,
    pub chips_y: u32,
    pub perms_head: u32,
    pub perms_btn: u32,
    pub bottom: u32,
}

/// `None` when the cursor is on nothing, which is the one case the panel has no
/// entry to describe.
pub fn info_geom(state: &State, win_w: u32, win_h: u32) -> Option<InfoGeom> {
    let entry = state.entries.get(state.cursor)?;
    let (m, pad, sec_h, sec_gap) = (metrics(), px(PAD), px(SEC_H), px(SEC_GAP));
    let (x, top) = (info_x(win_w) + pad, m.first_row_y);
    let w = m.info_w - pad * 2;
    let quick_head = top + px(82);
    let quick_y = quick_head + sec_h;
    let body_top = quick_y + px(QUICK_H) + sec_gap * 2;
    let tags_head = body_top + body_h(entry) + sec_gap;
    let chips_y = tags_head + sec_h;
    let names = state.tags.tags_for(entry.full_path.as_str());
    let perms_head = chips_y + band_h(&names, w) + sec_gap;
    Some(InfoGeom {
        x,
        w,
        title_y: top + px(14),
        loc_y: top + px(40),
        rule_y: top + px(70),
        quick_head,
        quick_y,
        body_top,
        tags_head,
        chips_y,
        perms_head,
        perms_btn: perms_head + sec_h,
        bottom: win_h.saturating_sub(m.footer_h + px(FOOT_PAD)),
    })
}
