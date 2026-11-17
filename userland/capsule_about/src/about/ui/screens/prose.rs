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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::Opaque;
use nonos_toolkit::widgets::text::{left, wrap_hard};

use super::super::metrics::metrics;
use super::super::mono::valid;

// How tall the wrapped run will be, walked with the same wrap the painter
// uses so a screen's reported extent and its drawn height cannot disagree.
pub fn height(b: &[u8], w: u32) -> u32 {
    let m = metrics();
    wrap_hard(valid(b), w, m.body_px).len() as u32 * m.pair_line
}

pub fn paint(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, b: &[u8], ink: Opaque) -> u32 {
    let (text, m) = (valid(b), metrics());
    let lines = wrap_hard(text, w, m.body_px);
    for (row, &(start, end)) in lines.iter().enumerate() {
        left(fb, x, y + (row as u32 * m.pair_line) as i32, &text[start..end], ink, m.body_px);
    }
    lines.len() as u32 * m.pair_line
}
