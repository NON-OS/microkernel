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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::size::CONTROL_H;
use nonos_toolkit::widgets::{Controls, Ink, Row};

use crate::browser::paint::pages::cells::{px, site_tile, TILE};
use crate::browser::paint::pages::rows::{self, place};
use crate::browser::shell::clock::ago;
use crate::browser::shell::listing::bookmark_rows;
use crate::browser::shell::site_label;
use crate::browser::shell::Shell;
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_key::key;
use crate::browser::ui::page_part::PagePart;

const NONE_YET: &str = "No bookmarks yet. The star in the address bar adds one.";

/// The Bookmarks list, newest first, each row opening its page, with when
/// it was added and a button that removes it. `now` is the uptime the
/// ages are counted to.
pub fn paint(
    shell: &Shell,
    g: &ListGeom,
    now: i64,
    fb: &mut PaintBuffer,
    c: &mut Controls<PagePart>,
) {
    let list = bookmark_rows(shell);
    if list.is_empty() {
        return rows::empty(fb, g, NONE_YET);
    }
    rows::frame(fb, g, list.len());
    for (i, &k) in list.iter().enumerate() {
        let (r, b) = (g.row_rect(i), &shell.bookmarks[k]);
        if !rows::shown(fb, g, r) {
            continue;
        }
        let (title, age) = (site_label::title(&b.title, &b.url), ago(now, b.at_ms));
        let row = Row::new(&title).note(&b.url).value(&age).value_ink(Ink::Muted);
        let row = row.leading(px(TILE) as u32).trailing(px(CONTROL_H) as u32);
        let id = key(&b.url, b.at_ms);
        let parts = (Some(PagePart::Open(k, id)), Some(PagePart::Remove(k, id)));
        let (lead, _) = place(fb, g, c, (r, row), parts);
        site_tile(fb, lead, &site_label::site(&b.url));
    }
}
