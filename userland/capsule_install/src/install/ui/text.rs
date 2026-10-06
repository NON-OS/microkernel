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

//! Text placement in the brand's faces: Geist for text, JetBrains Mono for
//! hashes and labels. Every line in this window goes through here, so the
//! metrics are decided once.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use nonos_brand::{label_w, line_h, measure, text, Face};

/// The size a footer's two hints are set at, in mono capitals: `px` when
/// both fit side by side in `room` with `gap` between them, else `small`.
/// The longest pair, a stoppable write's "Esc stop, the disk is left without
/// a table" with "do not power off", is wider than a 1280 pixel canvas's
/// column at caption size, and the two used to run into each other there.
pub fn keys_px(left: &str, right: &str, room: u32, gap: u32, px: f32, small: f32) -> f32 {
    let caps = |s: &str| -> String { s.chars().map(|c| c.to_ascii_uppercase()).collect() };
    let (l, r) = (caps(left), caps(right));
    if label_w(&l, px) + label_w(&r, px) + gap <= room {
        px
    } else {
        small
    }
}

pub fn line(fb: &mut PaintBuffer, x: u32, top: u32, s: &str, argb: u32, px: f32) {
    text(fb, x, top, s, Face::Body, argb, px);
}

/// A headline: Geist at 500.
pub fn title(fb: &mut PaintBuffer, x: u32, top: u32, s: &str, argb: u32, px: f32) {
    text(fb, x, top, s, Face::Headline, argb, px);
}

pub fn mono(fb: &mut PaintBuffer, x: u32, top: u32, s: &str, argb: u32, px: f32) {
    text(fb, x, top, s, Face::Mono, argb, px);
}

pub fn width(_fb: &PaintBuffer, s: &str, px: f32) -> u32 {
    measure(s, Face::Body, px, 0.0)
}

pub fn right(fb: &mut PaintBuffer, right_x: u32, top: u32, s: &str, argb: u32, px: f32) {
    let w = width(fb, s, px);
    line(fb, right_x.saturating_sub(w), top, s, argb, px);
}

/// The top of a line box centred in a row of height `h`.
pub fn top_of(y: u32, h: u32, px: f32) -> u32 {
    y + h.saturating_sub(line_h(Face::Body, px)) / 2
}

/// Longest prefix that measures within `max_w`, cut on a char boundary.
/// Never by glyph count: the face is proportional.
pub fn fit<'a>(fb: &PaintBuffer, s: &'a str, px: f32, max_w: u32) -> &'a str {
    let mut end = s.len();
    while end > 0 {
        if s.is_char_boundary(end) && width(fb, &s[..end], px) <= max_w {
            return &s[..end];
        }
        end -= 1;
    }
    ""
}
