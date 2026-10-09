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

/* The footer: the keys this screen takes, from `hints`, in mono capitals. */

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::hints::hints;
use super::metrics::Metrics;
use super::text::{keys_px, top_of};
use super::theme;
use crate::install::state::{Screen, State};
use nonos_brand::{label, label_w};

pub fn paint(fb: &mut PaintBuffer, m: &Metrics, state: &State, w: u32, h: u32) {
    let y = h - m.footer_h;
    fb.fill_rect(0, y, w, 1, theme::RULE);
    let (left, right_hint) = hints(state);
    let room = w.saturating_sub(2 * m.pad);
    let px = keys_px(left, right_hint, room, m.inset, m.small_px, m.label_px);
    let top = top_of(y, m.footer_h, px);
    keys(fb, m.pad, top, left, theme::MUTED, w - m.pad, px);
    let colour = if matches!(state.screen, Screen::Writing | Screen::Verifying) {
        theme::WARN
    } else {
        theme::FOREGROUND
    };
    keys(fb, 0, top, right_hint, colour, w - m.pad, px);
}

/// One side of the footer: left-aligned at `x`, or right-aligned to `right`
/// when `x` is zero.
pub fn keys(fb: &mut PaintBuffer, x: u32, top: u32, s: &str, argb: u32, right: u32, px: f32) {
    let caps: String = s.chars().map(|c| c.to_ascii_uppercase()).collect();
    let x = if x == 0 { right.saturating_sub(label_w(&caps, px)) } else { x };
    label(fb, x, top, &caps, argb, px);
}
