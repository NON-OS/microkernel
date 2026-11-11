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

//! Search: the live query echoed as a heading, then the filtered rows. With an
//! empty query the screen names what it searches rather than pretending to
//! have suggestions. With an address in the field, the page is about the
//! download instead.

extern crate alloc;

use alloc::string::String;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Kind, SectionHeader, State};

use crate::library::{Library, Queue};
use crate::ui::drawn::{glow, labelled};
use crate::ui::fmt::count;
use crate::ui::geometry::{page_head, Rect};
use crate::ui::hover::Lit;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::target::Target::{PageClear, PagePlay, Row};

use super::lib_geom::row_h;
use super::search_geom::{clear_rect, head_rect, list_rect, play_rect, visible};
use super::search_geom::{CLEAR, DOWNLOAD, PLAY};
use super::track_cols::header;
use super::track_row::row;

const ENTER_DOWNLOADS: &str =
    "Press Enter to download it into /home/nonos/music over the chosen network, then play it";

pub fn paint(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    queue: &Queue,
    hits: &[usize],
    query: &str,
    scroll: usize,
    playing: Option<usize>,
    level: u16,
    lit: &Lit,
) {
    if crate::fetch::is_address(query) {
        address(fb, icons, r, lit);
        return;
    }
    let title = if query.is_empty() {
        String::from("Search")
    } else {
        let mut s = String::from("\u{201c}");
        s.push_str(query);
        s.push('\u{201d}');
        s
    };
    let sub = if query.is_empty() {
        count(lib.tracks.len(), "track indexed", "tracks indexed")
    } else {
        count(hits.len(), "match", "matches")
    };
    SectionHeader::new(&title).note(&sub).draw(fb, page_head(r).tk());
    if !query.is_empty() {
        let (clear, play) = (lit.of(PageClear), lit.of(PagePlay));
        labelled(fb, icons, clear_rect(r), CLEAR, Some(Glyph::Close), Kind::Secondary, clear);
        if !hits.is_empty() {
            glow(fb, play_rect(r));
            labelled(fb, icons, play_rect(r), PLAY, Some(Glyph::Play), Kind::Primary, play);
        }
    }
    header(fb, head_rect(r));
    let (l, step) = (list_rect(r), row_h());
    for slot in 0..visible(r) {
        let Some(&idx) = hits.get(scroll + slot) else { break };
        let Some(t) = lib.get(idx) else { continue };
        let rr = Rect::new(l.x, l.y + slot as i32 * step, l.w, step);
        let look = State { selected: playing == Some(idx), ..lit.of(Row(scroll + slot)) };
        row(fb, icons, rr, (t, idx + 1), queue.contains(idx), look, level);
    }
}

/// An address in the field: the page says what Enter does with it, and how
/// the download is going once it runs, in place of matches it cannot have.
fn address(fb: &mut PaintBuffer, icons: &Icons, r: Rect, lit: &Lit) {
    let running = crate::fetch::line();
    let sub = if running.is_empty() { String::from(ENTER_DOWNLOADS) } else { running };
    SectionHeader::new("Download an MP3").note(&sub).draw(fb, page_head(r).tk());
    let (clear, play) = (lit.of(PageClear), lit.of(PagePlay));
    labelled(fb, icons, clear_rect(r), CLEAR, Some(Glyph::Close), Kind::Secondary, clear);
    glow(fb, play_rect(r));
    labelled(fb, icons, play_rect(r), DOWNLOAD, Some(Glyph::Download), Kind::Primary, play);
}
