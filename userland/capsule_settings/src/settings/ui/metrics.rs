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

//! Panel geometry.
//!
//! Every size here is a logical pixel, read once at the display scale through
//! `metrics()`. The AA text facade clamps at `ttf::MIN_UI_PX` (17.0) for draw
//! *and* measure. A caption asked for at 13px renders at 17px but measures at
//! 17px too, so rows sized for a smaller caption would overflow the box they
//! were measured into. The scale below starts at the floor and the rows are
//! tall enough to hold two lines of it.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::font::ttf::line_height;

const SIDEBAR_W: u32 = 212;
const NAV_TOP: u32 = 14;
const NAV_H: u32 = 38;
const NAV_GAP: u32 = 2;
const NAV_PAD_X: u32 = 12;

const PANE_PAD_X: u32 = 28;
const PANE_PAD_TOP: u32 = 24;

const HEAD_ICON: u32 = 30;
/// Air between the page icon and its title, and between title and subtitle.
const HEAD_ICON_GAP: u32 = 14;
const HEAD_LINE_GAP: u32 = 2;
const HEAD_H: u32 = 72;

const CARD_GAP: u32 = 16;
const CARD_PAD_X: u32 = 16;
const CARD_HEAD_H: u32 = 48;
const CARD_HEAD_NOTE_H: u32 = 74;

const ROW_H: u32 = 44;
const ROW_NOTE_H: u32 = 66;

const SWITCH_W: u32 = 42;
const SWITCH_H: u32 = 24;

const SLIDER_W: u32 = 150;

const PILL_H: u32 = 26;

const TITLE_PX: f32 = 26.0;
const SUBTITLE_PX: f32 = 17.0;
const BODY_PX: f32 = 17.0;
const NOTE_PX: f32 = 17.0;

/// Every panel size at the display scale in force, in drawing pixels.
pub struct Metrics {
    pub sidebar_w: u32,
    pub nav_top: u32,
    pub nav_h: u32,
    pub nav_gap: u32,
    pub nav_pad_x: u32,
    pub pane_pad_x: u32,
    pub pane_pad_top: u32,
    pub head_icon: u32,
    pub head_icon_gap: u32,
    pub head_line_gap: u32,
    pub head_h: u32,
    pub card_gap: u32,
    pub card_pad_x: u32,
    pub card_head_h: u32,
    pub card_head_note_h: u32,
    pub row_h: u32,
    pub row_note_h: u32,
    pub switch_w: u32,
    pub switch_h: u32,
    pub slider_w: u32,
    pub pill_h: u32,
    pub title_px: f32,
    pub subtitle_px: f32,
    pub body_px: f32,
    pub note_px: f32,
}

/// The panel sizes at the display scale in force: each logical size above
/// read once through `scale()`. The page head band is never shorter than the
/// title and subtitle it holds, at whatever text size is chosen.
pub fn metrics() -> Metrics {
    let s = scale();
    let (title_px, subtitle_px) = (s.font(TITLE_PX), s.font(SUBTITLE_PX));
    let head_line_gap = s.px(HEAD_LINE_GAP);
    let lines = line_height(title_px) + line_height(subtitle_px);
    let block = (lines.max(0) as u32 + head_line_gap).max(s.px(HEAD_ICON));
    Metrics {
        sidebar_w: s.px(SIDEBAR_W),
        nav_top: s.px(NAV_TOP),
        nav_h: s.px(NAV_H),
        nav_gap: s.px(NAV_GAP),
        nav_pad_x: s.px(NAV_PAD_X),
        pane_pad_x: s.px(PANE_PAD_X),
        pane_pad_top: s.px(PANE_PAD_TOP),
        head_icon: s.px(HEAD_ICON),
        head_icon_gap: s.px(HEAD_ICON_GAP),
        head_line_gap,
        head_h: s.px(HEAD_H).max(block),
        card_gap: s.px(CARD_GAP),
        card_pad_x: s.px(CARD_PAD_X),
        card_head_h: s.px(CARD_HEAD_H),
        card_head_note_h: s.px(CARD_HEAD_NOTE_H),
        row_h: s.px(ROW_H),
        row_note_h: s.px(ROW_NOTE_H),
        switch_w: s.px(SWITCH_W),
        switch_h: s.px(SWITCH_H),
        slider_w: s.px(SLIDER_W),
        pill_h: s.px(PILL_H),
        title_px,
        subtitle_px,
        body_px: s.font(BODY_PX),
        note_px: s.font(NOTE_PX),
    }
}
