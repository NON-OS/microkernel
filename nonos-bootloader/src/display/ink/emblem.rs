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

//! The emblem every screen opens with: the brand frame and the Ø, and two
//! mono captions under it.

use super::frame::draw_frame;
use super::label::label_centered;
use super::mark::draw_mark;
use super::palette::TEXT_3;
use super::scene::Scene;
use super::style::Style;
use super::text::metrics;

/// The frame in `color`, drawn to `progress` thousandths; once whole, the Ø
/// in `color` and lit, or in grey and unlit.
pub fn draw_emblem(s: &Scene, progress: u32, color: u32, lit: bool) {
    let (x, y, w, h) = s.frame;
    if progress > 0 {
        draw_frame(x, y, w, h, w * 20 / 100, (s.u / 3).max(2), color, progress);
    }
    if progress >= 1000 {
        let (c, glow) = if lit { (color, 190) } else { (TEXT_3, 0) };
        draw_mark(x + w / 2, y + h / 2, c, glow);
    }
}

/// Two caption lines under the frame: what the screen is, then a detail.
pub fn draw_captions(s: &Scene, first: &[u8], c1: u32, second: &[u8], c2: u32) {
    let (x, y, w, h) = s.frame;
    let mono = metrics(Style::Mono);
    let cy = y + h + mono.line * 2;
    label_centered(x, w, cy, first, c1);
    label_centered(x, w, cy + mono.line + s.u, second, c2);
}
