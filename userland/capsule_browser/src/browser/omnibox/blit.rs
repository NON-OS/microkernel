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

/* How a scroll repaints the page: shift the rows still in view and draw
 * only the rows that came into view. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blit {
    Same,
    /* Nothing on screen survives the move: repaint the whole page. */
    Full,
    Rows { src_y: u32, dst_y: u32, rows: u32, expose_y: u32, expose_h: u32 },
}

/* Rows are viewport rows, 0 at the top of the page area. */
pub fn blit_plan(old: u32, new: u32, view_h: u32) -> Blit {
    if old == new {
        return Blit::Same;
    }
    let d = old.abs_diff(new);
    if d >= view_h {
        return Blit::Full;
    }
    let rows = view_h - d;
    if new > old {
        Blit::Rows { src_y: d, dst_y: 0, rows, expose_y: rows, expose_h: d }
    } else {
        Blit::Rows { src_y: 0, dst_y: d, rows, expose_y: 0, expose_h: d }
    }
}

/* A row copy and up to two row bands to repaint after it. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shift {
    pub src_y: u32,
    pub dst_y: u32,
    pub rows: u32,
    pub bands: [(u32, u32); 2],
}

/* Keep the bottom `guard` rows out of the copy: they hold the hovered-link
 * bubble and the window's rounded corners, which stay put on screen while
 * the page moves, so they are always redrawn instead. None means the copy
 * would save nothing and the page should be repainted whole. */
pub fn guarded(plan: Blit, view_h: u32, guard: u32) -> Option<Shift> {
    let Blit::Rows { src_y, dst_y, rows, expose_y, expose_h } = plan else {
        return None;
    };
    let keep = view_h.saturating_sub(guard).saturating_sub(src_y.max(dst_y)).min(rows);
    if keep == 0 {
        return None;
    }
    let tail = (dst_y + keep, view_h);
    let head = if expose_y < tail.0 { (expose_y, expose_y + expose_h) } else { (0, 0) };
    Some(Shift { src_y, dst_y, rows: keep, bands: [head, tail] })
}
