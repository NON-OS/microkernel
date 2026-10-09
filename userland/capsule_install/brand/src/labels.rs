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

//! Labels the brand's way: JetBrains Mono capitals tracked wide, and the
//! section marker, a dot before a label.

use nonos_app_skeleton::PaintBuffer;

use super::faces::Face;
use super::type_set::{line_h, measure, spaced};

/// A label: mono capitals tracked at 0.3 em. `s` is already in capitals.
pub fn label(fb: &mut PaintBuffer, x: u32, top: u32, s: &str, argb: u32, px: f32) -> u32 {
    spaced(fb, x, top, s, Face::Mono, argb, px, px * 0.3)
}

pub fn label_w(s: &str, px: f32) -> u32 {
    measure(s, Face::Mono, px, px * 0.3)
}

/// A section marker: a dot in `lamp`, then the label.
pub fn marker(
    fb: &mut PaintBuffer,
    x: u32,
    top: u32,
    s: &str,
    lamp: u32,
    argb: u32,
    px: f32,
) -> u32 {
    let d = (px * 0.42) as u32;
    let lh = line_h(Face::Mono, px);
    fb.fill_round(x, top + lh.saturating_sub(d) / 2, d, d, d / 2, lamp);
    label(fb, x + d + (px * 0.8) as u32, top, s, argb, px)
}
