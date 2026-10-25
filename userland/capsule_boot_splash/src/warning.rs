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
//! The no-IOMMU warning: one red line, centred in its row under the panel,
//! in the owner's words for the posture.

use nonos_toolkit::font::atlas::FontAtlas;
use nonos_toolkit::font::render::draw_text;

use crate::layout::Area;

pub const TEXT: &[u8] = b"device DMA not confined by hardware";
pub const RED: u32 = 0xFFFF_3B3B;

pub(crate) fn draw(buf: &mut [u32], spx: usize, w: u32, h: u32, row: Area) {
    let x = w.saturating_sub(FontAtlas::default().text_width(TEXT)) / 2;
    draw_text(buf, spx, w, h, x, row.1, TEXT, RED);
}
