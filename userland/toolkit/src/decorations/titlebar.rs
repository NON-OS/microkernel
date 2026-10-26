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
//! The frame's measures at one to one (the design notes, Window
use super::controls::draw_controls_at;
use super::frame_rect::{frame_rect_at, margin_at, radius_at, titlebar_h_at};
use super::hit_test::DecorationHit;
use super::metrics::HAIRLINE_PX;
use super::palette::{focus_rule, frame_bg, frame_edge, SHADOW, TRANSPARENT};
use super::scale::{at, ONE};
use super::title::draw_title;
use crate::paint::PaintBuffer;

/// What the frame shows besides its size: the control under the pointer and
/// whether the window has the keyboard.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FrameState {
    pub hover: DecorationHit,
    pub focused: bool,
}

pub fn draw_frame(fb: &mut PaintBuffer, maximized: bool, title: &[u8], s: FrameState, aw: u32) {
    draw_frame_at(fb, maximized, title, s, aw, ONE);
}

/// The whole frame of a window `fb` covers, at `quarters` of display scale:
/// shadow, ground, edge, title bar, title, controls, and the focus rule.
pub fn draw_frame_at(
    fb: &mut PaintBuffer,
    maximized: bool,
    title: &[u8],
    state: FrameState,
    accessory_w: u32,
    quarters: u32,
) {
    let (w, h) = (fb.width, fb.height);
    let f = frame_rect_at(w, h, maximized, quarters);
    let r = radius_at(maximized, quarters);
    fb.clear(TRANSPARENT);
    fb.shadow_round(f.x, f.y, f.w, f.h, r, margin_at(maximized, quarters), SHADOW);
    fb.panel(f.x, f.y, f.w, f.h, r, frame_bg(), frame_edge());
    let bar = titlebar_h_at(quarters);
    fb.fill_rect(f.x, f.y + bar, f.w, at(HAIRLINE_PX, quarters), frame_edge());
    draw_controls_at(fb, maximized, state.hover, quarters);
    draw_title(fb, maximized, title, state.focused, accessory_w, quarters);
    if state.focused {
        draw_focus_rule_at(fb, maximized, quarters);
    }
}

/// The focused window's rule, along the straight part of the frame's top
/// edge and over it. A runtime that strokes the edge again after the app
/// paints (app_skeleton's finish) draws this after that stroke.
pub fn draw_focus_rule_at(fb: &mut PaintBuffer, maximized: bool, quarters: u32) {
    let f = frame_rect_at(fb.width, fb.height, maximized, quarters);
    let r = radius_at(maximized, quarters);
    let line = at(HAIRLINE_PX, quarters);
    fb.fill_rect(f.x + r, f.y, f.w.saturating_sub(r * 2), line, focus_rule());
}
