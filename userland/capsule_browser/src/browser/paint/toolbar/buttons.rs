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
use nonos_toolkit::widgets::{Controls, IconButton, Kind, Rect as Area};

use super::badge;
use super::look::{icon, Shows};
use crate::browser::ui::rect::Rect;
use crate::browser::ui::toolbar_geom::{Part, ToolbarGeom};

/// Lays out every toolbar control in `c` and draws the buttons. Back and
/// Forward are disabled with nowhere to go, the star with no page shown and
/// Reader with nothing to read; Reader is Primary while the reader shows.
/// The address field and, over it, the route badge are placed last.
pub fn paint(fb: &mut PaintBuffer, g: &ToolbarGeom, s: &Shows, c: &mut Controls<Part>) {
    c.clear();
    let live = [s.back, s.fwd, true, true, s.ready || s.reading, s.page, true, true];
    let parts = [Part::Back, Part::Fwd, Part::Reload, Part::Home];
    let rest = [Part::Reader, Part::Star, Part::Tabs, Part::Sidebar];
    let rects = [g.back, g.fwd, g.reload, g.home, g.reader, g.star, g.tabs, g.sidebar];
    for (i, part) in parts.into_iter().chain(rest).enumerate() {
        let at = area(rects[i]);
        let state = if live[i] { c.place(at, part) } else { c.place_disabled(at, part) };
        let kind = if part == Part::Reader && s.reading { Kind::Primary } else { Kind::Plain };
        if let Some(id) = icon(part, s) {
            IconButton::new(id).kind(kind).state(state).draw(fb, at);
        }
    }
    c.push(area(g.pill), Part::Pill);
    c.push(badge::body(g.route, s.route), Part::Route);
}

fn area(r: Rect) -> Area {
    Area { x: r.x.max(0) as u32, y: r.y.max(0) as u32, w: r.w.max(0) as u32, h: r.h.max(0) as u32 }
}
