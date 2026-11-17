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
use nonos_toolkit::widgets::Controls;

use crate::browser::paint::pages::{bookmarks, downloads, header, history, tab_grid};
use crate::browser::shell::cards::shown;
use crate::browser::shell::{Page, Shell};
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_part::PagePart;

/// A list page laid out in `g`, its controls laid out afresh in `c`: the
/// list at its scroll, then the header band over it. `now` is the uptime
/// bookmark ages are counted to.
pub fn draw(
    shell: &Shell,
    g: &ListGeom,
    now: i64,
    fb: &mut PaintBuffer,
    c: &mut Controls<PagePart>,
) {
    c.clear();
    let shown = shown(shell);
    match shell.page {
        Page::Tabs => tab_grid::paint(shell, &shown, g, fb, c),
        Page::Bookmarks => bookmarks::paint(shell, g, now, fb, c),
        Page::Downloads => downloads::paint(shell, g, fb, c),
        _ => history::paint(shell, g, fb, c),
    }
    header::paint(shell, shown.len(), g, fb, c);
}
