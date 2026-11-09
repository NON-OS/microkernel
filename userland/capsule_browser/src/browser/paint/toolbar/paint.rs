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
use nonos_toolkit::theme::roles;

use crate::browser::event::reader_ready;
use crate::browser::paint::toolbar::look::Shows;
use crate::browser::paint::toolbar::{buttons, pill as field, route::route_of};
use crate::browser::shell::{bookmarks, Page, Shell};
use crate::browser::state::{State, View};
use crate::browser::ui::rect::Rect;
use crate::browser::ui::toolbar_geom::{self, ToolbarGeom};

/* The toolbar: a strip with a hairline under it in the OS theme's roles,
 * Back and Forward (dimmed when there is nowhere to go), Reload (Stop
 * while a page loads), Home, the address pill with its route badge, the
 * bookmark star and the sidebar button, all laid out afresh in the
 * toolbar's controls. */
pub fn paint(state: &mut State, shell: &Shell, fb: &mut PaintBuffer) {
    let r = roles();
    fb.fill_rect(0, 0, fb.width, fb.height, r.surface.argb());
    fb.fill_rect(0, fb.height.saturating_sub(1), fb.width, 1, r.border.argb());
    let g = geom(fb);
    let s = shows(state, shell);
    buttons::paint(fb, &g, &s, &mut state.ui.bar);
    field::paint(state, &g, fb);
}

/* Back is live on the home page when there is a page to return to, and on
 * a page the window draws shown over a loaded one. The star is live while
 * a page is shown, Reader while the tab has a parsed page or the reader is
 * open. */
fn shows(state: &State, shell: &Shell) -> Shows {
    let h = &state.ui.history;
    let home = state.view == View::Home && h.current().is_some();
    let page = state.view == View::Page;
    Shows {
        back: h.can_back() || home || (state.on_own && page),
        fwd: h.can_forward(),
        loading: state.loading(),
        page,
        marked: page && bookmarks::contains(&shell.bookmarks, &state.ui.current_url),
        ready: reader_ready(state),
        reading: shell.page == Page::Reader,
        route: route_of(state),
    }
}

/* The address pill alone: it repaints on its own for every keystroke, so
 * typing redraws this and nothing else. */
pub fn pill(state: &State, fb: &mut PaintBuffer) {
    let g = geom(fb);
    field::paint(state, &g, fb);
}

fn geom(fb: &PaintBuffer) -> ToolbarGeom {
    toolbar_geom::buttons(Rect::new(0, 0, fb.width as i32, fb.height as i32))
}
