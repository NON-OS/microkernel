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
 * The Qwen step's list: "None for now", then only the tiers that fit this
 * machine, smallest first, one line each with its download and the memory
 * it needs to run. A tier that does not fit is not a row; one line under
 * the list says how many there are, as the store's tier cards do, so the
 * two never disagree about what this machine can run.
 *
 * Where the canvas has room for fewer rows than there are, the list shows
 * a run of them that keeps the chosen row in view, and a line under it
 * says which rows those are; Up and Down reach every one of them.
 */

use alloc::format;

use crate::render::ink::{draw_label, draw_text};

use crate::qwen::{label, need_of, For, QwenState, STICK_TIER};
use crate::render::layout::{layout, window, QWEN_CELLS};
use crate::render::paint::fill_rect;
use crate::render::theme::{ACCENT, FG, HINT, ROW_SEL_BG};
use crate::render::widgets::text::{cat, size};

/* Draws at most `room` rows from `y`; returns the y below them. */
pub fn draw(
    buf: &mut [u32],
    spx: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    q: &QwenState,
    install: bool,
    room: u32,
) -> u32 {
    let l = layout(w, h);
    let rows = q.open(install) as u32 + 1;
    let sel = q.row(install);
    let (first, shown) = window(rows, room, sel as u32);
    let mut put = |i: usize, cells: [&[u8]; 3]| {
        let Some(at) = (i as u32).checked_sub(first).filter(|&at| at < shown) else { return };
        let top = y + l.line_h * at;
        if sel as usize == i {
            fill_rect(buf, spx, w, h, x, top, l.list_w, l.line_h, ROW_SEL_BG);
            draw_text(buf, spx, w, h, x + l.unit, top, b">", ACCENT);
        }
        for (cell, at) in cells.iter().zip(QWEN_CELLS) {
            draw_text(buf, spx, w, h, x + at * l.unit, top, cell, FG);
        }
    };
    put(0, [b"None for now", b"", b""]);
    for row in 1..rows as u8 {
        let Some((tier, bytes)) = q.tier_at(install, row) else { break };
        let (mut s, mut n, mut note) = ([0u8; 24], [0u8; 24], [0u8; 48]);
        let need = core::str::from_utf8(tier).ok().and_then(|t| need_of(t, bytes)).unwrap_or(0);
        let stick: &[u8] = if tier == STICK_TIER.as_bytes() { b", on this stick" } else { b"" };
        let note = cat(&mut note, &[b"needs ", size(need, &mut n), stick]);
        put(row as usize, [label(tier), size(bytes, &mut s), note]);
    }
    let mut end = y + l.line_h * shown;
    if shown < rows {
        let s = format!("rows {} to {} of {rows}, Up and Down scroll", first + 1, first + shown);
        draw_label(buf, spx, w, h, x, end + l.unit / 2, s.as_bytes(), HINT);
        end += l.line_h;
    }
    if let Some(line) = hidden_line(q, install) {
        draw_label(buf, spx, w, h, x, end + l.unit / 2, line.as_bytes(), HINT);
        end += l.line_h;
    }
    end
}

/* "7 larger tiers need more memory than this machine has", when any do. */
fn hidden_line(q: &QwenState, install: bool) -> Option<alloc::string::String> {
    let n = q.hidden(install);
    if n == 0 || q.who(install) == For::NoDisk {
        return None;
    }
    Some(match (n, q.memory) {
        (_, None) => format!("{n} tiers not listed: this machine's memory could not be read"),
        (1, _) => alloc::string::String::from("1 larger tier needs more memory than this machine has"),
        (n, _) => format!("{n} larger tiers need more memory than this machine has"),
    })
}
