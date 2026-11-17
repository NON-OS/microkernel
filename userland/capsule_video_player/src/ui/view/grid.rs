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

use nonos_app_skeleton::paint::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Col, Rect, Scrollbar, State, Table};

use super::list_geom::{grid_capacity, list_at, row_rect, rows_visible, tile_rect, track};
use super::tile::paint_tile;
use crate::app::state::VideoApp;
use crate::ui::format::{duration, resolution, size};
use crate::ui::target::Target;

pub const COLUMNS: [Col; 5] = [
    Col::new("NAME").share(44),
    Col::new("DURATION").share(14),
    Col::new("RESOLUTION").share(16),
    Col::new("SIZE").share(13),
    Col::new("KIND").share(13),
];

/// The tiles on the page; the keyboard's cursor is selected and focused.
pub fn paint_grid(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    let browse = &app.browse;
    for slot in 0..browse.len().saturating_sub(browse.scroll).min(grid_capacity(body)) {
        let index = browse.scroll + slot;
        let Some(item) = browse.get(index) else { break };
        let at = index == browse.sel;
        let look = State { selected: at, focused: at, ..app.look(Target::Tile(index)) };
        paint_tile(fb, tile_rect(body, slot), item, look);
    }
}

/// The rows on the page as the shared Table, from the first on the page; the
/// keyboard's cursor is selected and ringed, and the row under the pointer lit.
pub fn paint_list(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    let (browse, at) = (&app.browse, list_at(body));
    let (first, visible) = (browse.scroll.min(browse.len()), rows_visible(at));
    let shown = (first..browse.len()).take(visible).filter_map(|i| browse.get(i));
    let cells: Vec<[String; 5]> = shown
        .map(|m| {
            let dims = resolution(m.width, m.height);
            [m.title().into(), duration(m.duration_ms), dims, size(m.size), m.kind.label().into()]
        })
        .collect();
    let lines: Vec<[&str; 5]> = cells.iter().map(|c| c.each_ref().map(String::as_str)).collect();
    let rows: Vec<&[&str]> = lines.iter().map(|l| &l[..]).collect();
    let lit = match app.hover.over {
        Some(Target::Row(i)) => Some((i, app.look(Target::Row(i)))),
        _ => None,
    };
    let hovered = lit.filter(|(_, l)| l.hovered).map(|(i, _)| i);
    let pressed = lit.filter(|(_, l)| l.pressed).map(|(i, _)| i);
    let table = Table::new(&COLUMNS).rows(&rows).first(first).selected(Some(browse.sel));
    table.hovered(hovered).pressed(pressed).draw(fb, at);
    if let Some(k) = browse.sel.checked_sub(first).filter(|&k| k < rows.len()) {
        let ring = scale().hair() * 2;
        draw::outline(fb, row_rect(at, k), 0, ring, roles().focus_ring);
    }
    let (scroll, len) = (browse.scroll as u32, browse.len() as u32);
    Scrollbar::new(scroll, len, visible as u32).draw(fb, track(body));
}
