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
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::text::clip;
use nonos_toolkit::widgets::{draw, Scrollbar, Table};

use crate::pm::state::notes::empty_table;
use crate::pm::state::State;

use super::chrome::Rect;
use super::hover_look;
use super::metrics::{BODY_PX, SCROLL_PAD, SCROLL_W};
use super::table_geom::{self, head_h, Col};
use super::{table_cell, text};

// The shared table draws the header with its sort chevron and the visible
// slice, the selection, hover and press lit. Every offset inside it comes from
// table_geom, which is also what the hit test reads, so a click cannot land on
// a row or a column other than the one drawn under it.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect, cols: &[Col]) {
    let rows = state.filtered();
    let first = state.scroll.min(rows.len());
    let shown = &rows[first..first + table_geom::shown(first, rows.len(), r.h)];
    let ((specs, fixed), (hovered, pressed)) = (table_geom::specs(cols), hover_look::row(state));
    let spans = table_geom::spans(&fixed, r.w);
    let cells = table_cell::cells(state, (cols, &spans), shown);
    let lines: Vec<Vec<&str>> =
        cells.iter().map(|c| c.iter().map(String::as_str).collect()).collect();
    let body: Vec<&[&str]> = lines.iter().map(Vec::as_slice).collect();
    let pid = state.selected_pid;
    let selected = rows.iter().position(|row| pid != 0 && row.pid == pid);
    let mut table = Table::new(&specs).rows(&body).first(state.scroll).selected(selected);
    if let Some((col, up)) = table_geom::sorted(cols, state.sort) {
        table = table.sort(col, up);
    }
    table.hovered(hovered).pressed(pressed).draw(fb, *r);
    table_cell::marks(state, fb, r, (cols, &spans), shown, &cells);
    head_hover(state, fb, r, cols, &spans);
    /*
     * Nothing to draw is never a silent blank card: the table says whether it
     * is still being read, could not be read, or is filtered to nothing.
     */
    if let Some(note) = empty_table(state.rows.len(), rows.len(), state.status) {
        let top = text::centred_top(r.y + head_h(), r.h.saturating_sub(head_h()), BODY_PX);
        let cut = clip(text::valid(note), r.w.saturating_sub(SCROLL_PAD * 2), BODY_PX).as_bytes();
        let x = r.x + r.w.saturating_sub(text::width(fb, cut, BODY_PX)) / 2;
        text::left(fb, x, top, cut, roles().muted, BODY_PX);
    }
    scrollbar(fb, r, state.scroll, rows.len(), table_geom::visible_rows(r.h));
}

// A header that sorts lights under the pointer as a row does. The shared table
// lights rows alone, so the wash goes over the header cell once it is drawn.
fn head_hover(state: &State, fb: &mut PaintBuffer, r: &Rect, cols: &[Col], spans: &[(u32, u32)]) {
    for (col, (x, w)) in cols.iter().zip(spans) {
        if table_geom::sort_for(*col).is_some() && hover_look::head(state, *col) {
            let cell = Rect { x: r.x + x, y: r.y, w: *w, h: table_geom::row_h() };
            draw::blend(fb, cell, roles().hover_wash);
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
    let x = r.x + r.w.saturating_sub(SCROLL_PAD + SCROLL_W);
    let track = Rect { x, y: r.y + head_h(), w: SCROLL_W, h: r.h - head_h() };
    Scrollbar::new(scroll as u32, total as u32, visible as u32).draw(fb, track);
}
