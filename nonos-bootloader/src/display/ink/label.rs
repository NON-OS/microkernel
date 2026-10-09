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

//! Labels the brand's way: JetBrains Mono, capitals, tracked wide, and a
//! section marker, a cyan dot before the label.

use super::parts::{draw_tracked, tracked_width};
use super::style::Style;
use super::text::metrics;
use super::dot;

fn track() -> u32 {
    (metrics(Style::Mono).px * 3 / 10).max(2)
}

/// Draw `s`, already in capitals, tracked; returns its width.
pub fn label(x: u32, y: u32, s: &[u8], color: u32) -> u32 {
    draw_tracked(x, y, s, Style::Mono, color, track())
}

pub fn label_width(s: &[u8]) -> u32 {
    tracked_width(s, Style::Mono, track())
}

/// A section marker: a dot in `lamp`, then the label. Returns its width.
pub fn marker(x: u32, y: u32, s: &[u8], lamp: u32, color: u32) -> u32 {
    let d = dot(x, y, Style::Mono, lamp);
    let gap = d + metrics(Style::Mono).px;
    gap + label(x + gap, y, s, color)
}

/// `label`, centred in the column at `x`, `w` wide.
pub fn label_centered(x: u32, w: u32, y: u32, s: &[u8], color: u32) {
    label(x + w.saturating_sub(label_width(s)) / 2, y, s, color);
}
