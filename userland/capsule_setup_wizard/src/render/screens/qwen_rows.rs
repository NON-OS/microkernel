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

/*
 * The Qwen step's list: "None for now", then every pinned tier smallest
 * first, one line each with its size and whether it fits this machine.
 * Rows that do not fit are dimmed and cannot be chosen.
 */

use nonos_toolkit::font::render::draw_text;

use crate::qwen::{label, QwenState, PINNED};
use crate::render::paint::fill_rect;
use crate::render::theme::{ACCENT, FG, HINT, ROW_SEL_BG};
use crate::render::widgets::text::{cat, size};

const ROW: u32 = 20;

/* Draws the list from `y`; returns the y below it. */
pub fn draw(buf: &mut [u32], spx: usize, w: u32, h: u32, x: u32, y: u32, q: &QwenState) -> u32 {
    let mut put = |i: usize, cells: [&[u8]; 3], fits: bool| {
        let top = y + ROW * i as u32;
        let color = if fits { FG } else { HINT };
        if q.sel as usize == i {
            fill_rect(buf, spx, w, h, x, top - 5, 360, ROW - 2, ROW_SEL_BG);
            draw_text(buf, spx, w, h, x + 8, top, b">", ACCENT);
        }
        for (cell, dx) in cells.iter().zip([24, 200, 280]) {
            draw_text(buf, spx, w, h, x + dx, top, cell, color);
        }
    };
    put(0, [b"None for now", b"", b""], true);
    let mut said = [0u8; 48];
    let too_big = too_big_for(q.memory, &mut said);
    for (i, (tier, bytes)) in PINNED.iter().enumerate() {
        let fits = i < q.fit as usize;
        let mut s = [0u8; 24];
        put(i + 1, [label(tier), size(*bytes, &mut s), if fits { b"fits" } else { too_big }], fits);
    }
    y + ROW * (PINNED.len() as u32 + 1)
}

fn too_big_for(memory: Option<u64>, out: &mut [u8; 48]) -> &[u8] {
    let Some(m) = memory else {
        return b"memory unknown";
    };
    let mut s = [0u8; 24];
    cat(out, &[b"too big for ", size(m, &mut s)])
}
