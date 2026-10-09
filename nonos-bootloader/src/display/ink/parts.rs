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

//! Small parts every screen shares: key caps, status dots, tracked titles.

use super::palette::{BORDER, RAISED, TEXT_2};
use super::card::card;
use super::shape::round_rect;
use super::style::{unit, Style};
use super::text::{draw, metrics, width};

/// A key cap with `label`, its top left at (x, y), a caption line and a
/// unit tall. Returns its width.
pub fn keycap(x: u32, y: u32, label: &[u8]) -> u32 {
    let u = unit();
    let m = metrics(Style::Caption);
    let w = keycap_width(label);
    card(x, y, w, m.line + u, u / 2 + 1, RAISED, BORDER);
    let tx = x + w.saturating_sub(width(label, Style::Caption)) / 2;
    draw(tx, y + u / 2, label, Style::Caption, TEXT_2);
    w
}

pub fn keycap_width(label: &[u8]) -> u32 {
    let u = unit();
    (width(label, Style::Caption) + 3 * u).max(metrics(Style::Caption).line + u)
}

/// A round status dot of `color`, centred vertically on a line of `style`.
pub fn dot(x: u32, y: u32, style: Style, color: u32) -> u32 {
    let m = metrics(style);
    let d = (m.px / 2).max(6) & !1;
    /* Centred on the x-height, about 0.27 em above the baseline. */
    let mid = m.ascent.saturating_sub(m.px * 27 / 100);
    round_rect(x, y + mid.saturating_sub(d / 2), d, d, d / 2, color);
    d
}

/// `s` with `track` extra pixels after each character. Returns its width.
pub fn draw_tracked(x: u32, y: u32, s: &[u8], style: Style, c: u32, track: u32) -> u32 {
    let mut cx = x;
    for ch in s.chunks(1) {
        cx += draw(cx, y, ch, style, c) + track;
    }
    cx.saturating_sub(x + track)
}

/// The width `draw_tracked` would take.
pub fn tracked_width(s: &[u8], style: Style, track: u32) -> u32 {
    s.chunks(1).map(|ch| width(ch, style) + track).sum::<u32>().saturating_sub(track)
}
