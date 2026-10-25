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

//! The splash's own title, NONOS's mark above the attestation panel.

use nonos_toolkit::font::atlas::FontAtlas;
use nonos_toolkit::font::render::{draw_text, draw_text_scaled};

const DIM: u32 = 0xFF5B_6B78;
const ACCENT: u32 = 0xFF00_D4AA;
const ACCENT_DIM: u32 = 0xFF00_5544;
const TITLE_SCALE: u32 = 5;

pub(crate) fn draw(buf: &mut [u32], spx: usize, w: u32, h: u32) {
    let atlas = FontAtlas::default();
    let title = b"N\xd8NOS";
    let tw = atlas.text_width(title) * TITLE_SCALE;
    let tx = w.saturating_sub(tw) / 2;
    let ty = h / 2 - 132;
    draw_text_scaled(buf, spx, w, h, tx.saturating_sub(1), ty, title, ACCENT_DIM, TITLE_SCALE);
    draw_text_scaled(buf, spx, w, h, tx, ty, title, ACCENT, TITLE_SCALE);
    let sub = b"zero-state attestation boot";
    let sx = w.saturating_sub(atlas.text_width(sub)) / 2;
    draw_text(buf, spx, w, h, sx, ty + TITLE_SCALE * 8 + 10, sub, DIM);
}
