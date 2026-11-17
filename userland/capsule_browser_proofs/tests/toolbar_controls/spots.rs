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

use capsule_browser_proofs::browser::toolbar::{badge, look::Shows};
use capsule_browser_proofs::browser::ui::toolbar_geom::Part;
use nonos_toolkit::widgets::Rect as Area;

use super::kit::geom;

/// Every control drawn as a widget: the eight buttons and the route badge.
pub const DRAWN: [Part; 9] = [
    Part::Back,
    Part::Fwd,
    Part::Reload,
    Part::Home,
    Part::Route,
    Part::Reader,
    Part::Star,
    Part::Tabs,
    Part::Sidebar,
];

/// The centre of the box `part` is drawn in, the badge's own body for the route.
pub fn centre(part: Part, s: &Shows) -> (i32, i32) {
    let g = geom();
    let r = match part {
        Part::Back => g.back,
        Part::Fwd => g.fwd,
        Part::Reload => g.reload,
        Part::Home => g.home,
        Part::Reader => g.reader,
        Part::Star => g.star,
        Part::Tabs => g.tabs,
        Part::Sidebar => g.sidebar,
        Part::Pill => g.text,
        Part::Route => return mid(badge::body(g.route, s.route)),
    };
    (r.x + r.w / 2, r.y + r.h / 2)
}

fn mid(b: Area) -> (i32, i32) {
    ((b.x + b.w / 2) as i32, (b.y + b.h / 2) as i32)
}
