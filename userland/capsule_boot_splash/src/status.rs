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

//! The status line: a spinner, the label and a blinking cursor, repainted
//! whole within its band (the layout's `status`) on every frame, and only there,
//! so damage for that band alone shows each frame whole.

use nonos_toolkit::font::atlas::FontAtlas;
use nonos_toolkit::font::render::draw_text;

use crate::layout::Area;

const DIM: u32 = 0xFF5B_6B78;
const ACCENT: u32 = 0xFF00_D4AA;
const SPIN: &[u8] = b"|/-\\";

pub(crate) fn status(buf: &mut [u32], spx: usize, w: u32, h: u32, band: Area, frame: u32) {
    let (_, y, _, rows) = band;
    crate::vignette::fill_band(buf, spx, w, h, y, rows);
    let atlas = FontAtlas::default();
    let sp = [SPIN[(frame % 4) as usize]];
    let cur: &[u8] = if frame & 1 == 0 { b"_" } else { b" " };
    let label = b"initializing zero-state";
    let x = w.saturating_sub(atlas.text_width(label) + 24) / 2;
    draw_text(buf, spx, w, h, x, y, &sp, ACCENT);
    draw_text(buf, spx, w, h, x + 16, y, label, DIM);
    draw_text(buf, spx, w, h, x + 16 + atlas.text_width(label) + 4, y, cur, ACCENT);
}
