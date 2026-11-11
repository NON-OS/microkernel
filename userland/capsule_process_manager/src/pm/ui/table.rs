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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::clip;
use nonos_toolkit::widgets::{draw, Scrollbar, Table};

use crate::pm::state::notes::empty_table;
use crate::pm::state::State;

use super::chrome::Rect;
use super::hover_look;
use super::metrics::metrics;
use super::table_geom::{self, head_h, Col};
use super::{table_cell, text};

const SCROLL_W: u32 = 4;
const SCROLL_PAD: u32 = 5;

// The shared table draws the header with its sort chevron and the visible
// slice, the selection, hover and press lit. Every offset inside it comes from
// table_geom, which is also what the hit test reads, so a click cannot land on
// a row or a column other than the one drawn under it.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect, cols: &[Col]) {
    let rows = state.filtered();
    let first = state.scroll.min(rows.len());
    let shown = &rows[first..first + table_geom::shown(first, rows.len(), r.h)];
    let (specs, (hovered, pressed)) = (table_geom::specs(cols), hover_look::row(state));
    let cells = table_cell::cells(state, (cols, &specs), *r, shown);
    let lines: Vec<Vec<&str>> =
        cells.iter().map(|c| c.iter().map(String::as_str).collect()).collect();
    let body: Vec<&[&str]> = lines.iter().map(Vec::as_slice).collect();
    let pid = state.selected_pid;
    let selected = rows.iter().position(|row| pid != 0 && row.pid == pid);
    let mut table = Table::new(&specs).rows(&body).first(state.scroll).selected(selected);
    if let Some((col, up)) = table_geom::sorted(cols, state.sort) {
        table = table.sort(col, up);
    }
    let table = table.hovered(hovered).pressed(pressed);
    table.draw(fb, *r);
    table_cell::marks(state, fb, (&table, *r), (cols, first), shown, &cells);
    head_hover(state, fb, (&table, *r), cols);
    /*
     * Nothing to draw is never a silent blank card: the table says whether it
     * is still being read, could not be read, or is filtered to nothing.
     */
    if let Some(note) = empty_table(state.rows.len(), rows.len(), state.status) {
        let (px, pad) = (metrics().body_px, scale().px(SCROLL_PAD));
        let top = text::centred_top(r.y + head_h(), r.h.saturating_sub(head_h()), px);
        let cut = clip(text::valid(note), r.w.saturating_sub(pad * 2), px).as_bytes();
        let x = r.x + r.w.saturating_sub(text::width(fb, cut, px)) / 2;
        text::left(fb, x, top, cut, roles().muted, px);
    }
    scrollbar(fb, r, state.scroll, rows.len(), table_geom::visible_rows(r.h));
}

// A header that sorts lights under the pointer as a row does. The shared table
// lights rows alone, so the wash goes over the header cell once it is drawn.
fn head_hover(state: &State, fb: &mut PaintBuffer, (table, r): (&Table, Rect), cols: &[Col]) {
    for (c, col) in cols.iter().enumerate() {
        if table_geom::sort_for(*col).is_some() && hover_look::head(state, *col) {
            if let Some(cell) = table.head_rect(r, c) {
                draw::blend(fb, cell, roles().hover_wash);
            }
        }
    }
}

// Drawn only when the list is taller than the card: the thumb's height is the
// visible share of the whole list and its travel is the real scroll offset, so
// the bar reports the position rather than decorating the edge. The toolkit
// bar, in rows, draws nothing while the list fits.
fn scrollbar(fb: &mut PaintBuffer, r: &Rect, scroll: usize, total: usize, visible: usize) {
    if r.h <= head_h() {
        return;
    }
    let (s, y) = (scale(), r.y + head_h());
    let x = r.x + r.w.saturating_sub(s.px(SCROLL_PAD) + s.px(SCROLL_W));
    let track = Rect { x, y, w: s.px(SCROLL_W), h: r.h - head_h() };

    Scrollbar::new(scroll as u32, total as u32, visible as u32).draw(fb, track);
}
