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

//! Library: a tab bar over a scrolling track table.

extern crate alloc;

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{SectionHeader, Separator, State, Tabs};

use crate::library::{by_album, by_artist, Library, Queue};
use crate::ui::fmt::count;
use crate::ui::geometry::{page_head, Rect};
use crate::ui::hover::Lit;
use crate::ui::icon::Icons;
use crate::ui::metrics::hair;
use crate::ui::state::LIB_TABS;
use crate::ui::target::Target;

use super::lib_geom::{head_rect, list_rect, row_h, tabs_rect, visible};
use super::track_cols::header;
use super::track_row::row;

pub fn paint(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    queue: &Queue,
    rows: &[usize],
    tab: usize,
    scroll: usize,
    playing: Option<usize>,
    level: u16,
    lit: &Lit,
) {
    let sub = count(rows.len(), "track", "tracks");
    SectionHeader::new("Library").note(&sub).draw(fb, page_head(r).tk());
    let hovered = match lit.0.over {
        Some(Target::Tab(i)) => Some(i),
        _ => None,
    };
    let strip = tabs_rect(r);
    Tabs::new(&LIB_TABS, tab).hovered(hovered).draw(fb, strip.tk());
    let hr = hair();
    Separator::new().draw(fb, Rect::new(strip.x, strip.bottom() - hr, strip.w, hr).tk());
    header(fb, head_rect(r));
    let (l, step) = (list_rect(r), row_h());
    for slot in 0..visible(r) {
        let Some(&idx) = rows.get(scroll + slot) else { break };
        let Some(t) = lib.get(idx) else { continue };
        let rr = Rect::new(l.x, l.y + slot as i32 * step, l.w, step);
        let look = State { selected: playing == Some(idx), ..lit.of(Target::Row(scroll + slot)) };
        // Its place in the list shown: under Artists or Albums the library's
        // own order would number the rows out of sequence.
        row(fb, icons, rr, (t, scroll + slot + 1), queue.contains(idx), look, level);
    }
}

pub fn rows_for(lib: &Library, queue: &Queue, tab: usize) -> Vec<usize> {
    match tab {
        1 => by_artist(&lib.tracks),
        2 => by_album(&lib.tracks),
        3 => queue.items().to_vec(),
        _ => (0..lib.tracks.len()).collect(),
    }
}
