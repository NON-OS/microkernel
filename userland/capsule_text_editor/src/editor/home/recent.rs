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

//! The document card. Rows come from the VFS listing (or the session's
//! most-recently-opened list) and open on a press; the note is the store
//! path, the only thing about a document the editor actually knows. "View
//! all" leads back to every document, and draws unavailable while that is
//! the list already shown.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Card, Controls, Rect, Row, Separator, State};

use super::doc_ref::{empty_line, section_title, DocRef};
use super::metrics_pane::{doc_row, list_card, view_all};
use super::pick::HomePick;

pub(super) fn paint_recent(
    fb: &mut PaintBuffer,
    nav: usize,
    list: &[DocRef],
    c: &mut Controls<HomePick>,
) {
    let title = section_title(nav);
    let (at, shown) = list_card(fb.width, fb.height, title, list.len());
    let card = Card::new().title(title);
    card.draw(fb, at);
    let (button, b_at) = view_all(at, title);
    let live = nav != 0;
    let st = if live {
        c.place(b_at, HomePick::ViewAll)
    } else {
        c.place_disabled(b_at, HomePick::ViewAll)
    };
    button.state(st).draw(fb, b_at);
    let content = card.content(at);
    if shown == 0 {
        let off = State { disabled: true, ..State::default() };
        Row::new(empty_line(nav)).state(off).draw(fb, doc_row(content, 0));
        return;
    }
    let hair = scale().hair();
    for (i, doc) in list.iter().take(shown).enumerate() {
        let row = doc_row(content, i);
        let st = c.place(row, HomePick::Doc(i));
        Row::new(&doc.name).note(&doc.path).state(st).draw(fb, row);
        if i > 0 {
            let line =
                Rect { x: row.x + hair, y: row.y, w: row.w.saturating_sub(hair * 2), h: hair };
            Separator::new().soft().draw(fb, line);
        }
    }
}
