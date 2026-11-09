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

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::{Controls, SectionHeader, State};

use crate::browser::paint::pages::cells::area;
use crate::browser::paint::pages::{group_head, pills};
use crate::browser::paint::tokens::palette;
use crate::browser::shell::{Page, Shell};
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_part::PagePart;

/// The header band over a list page: it covers the rows scrolled under it,
/// then carries the page title over a muted line and the filter pills, the
/// one in force selected and New group disabled once the window is full.
pub fn paint(
    shell: &Shell,
    tabs: usize,
    g: &ListGeom,
    fb: &mut PaintBuffer,
    c: &mut Controls<PagePart>,
) {
    fb.fill_rect(0, 0, fb.width, g.band.max(0) as u32, palette().ground);
    match group_head::filtered(shell) {
        Some(gr) => group_head::paint(shell, gr, tabs, g, fb, c),
        None => {
            let (title, sub) = heading(shell, tabs);
            SectionHeader::new(title).note(&sub).draw(fb, area(g.head));
        }
    }
    let (specs, widths) = pills::row(shell, g);
    for (i, p) in specs.iter().enumerate() {
        let (at, part) = (area(g.pill(&widths, i)), PagePart::Pill(p.pick));
        let st = if p.dim { c.place_disabled(at, part) } else { c.place(at, part) };
        p.widget().state(State { selected: p.on, ..st }).draw(fb, at);
    }
}

fn heading(shell: &Shell, tabs: usize) -> (&'static str, String) {
    match shell.page {
        Page::Tabs => ("Tabs", format!("{} open in this window", tabs)),
        Page::Bookmarks => ("Bookmarks", format!("{} saved", shell.bookmarks.len())),
        Page::Downloads => ("Downloads", String::from("Saved files go to /home/Downloads")),
        _ => ("History", String::from("Pages visited this session. History clears at shutdown")),
    }
}
