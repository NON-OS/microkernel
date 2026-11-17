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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{ICON, SPACE};
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Controls, Ink, Label, Rect as Area, Row};

use crate::browser::paint::pages::cells::{px, site_tile, TILE};
use crate::browser::paint::pages::rows::{self, place};
use crate::browser::shell::clock::hhmm;
use crate::browser::shell::listing::history_rows;
use crate::browser::shell::route::Route;
use crate::browser::shell::{site_label, Shell};
use crate::browser::ui::list_geom::ListGeom;
use crate::browser::ui::page_key::key;
use crate::browser::ui::page_part::PagePart;

const NONE_YET: &str = "No pages visited yet this session.";
const NONE_HERE: &str = "No pages visited on this route.";

/// The History list, newest first, under the route filter in force: each
/// row opens its page and shows the time and the route it came by.
pub fn paint(shell: &Shell, g: &ListGeom, fb: &mut PaintBuffer, c: &mut Controls<PagePart>) {
    let list = history_rows(shell);
    if list.is_empty() {
        let msg = if shell.visits.is_empty() { NONE_YET } else { NONE_HERE };
        return rows::empty(fb, g, msg);
    }
    rows::frame(fb, g, list.len());
    for (i, &k) in list.iter().enumerate() {
        let (r, v) = (g.row_rect(i), &shell.visits[k]);
        if !rows::shown(fb, g, r) {
            continue;
        }
        let (title, time) = (site_label::title(&v.title, &v.url), hhmm(v.at_ms));
        let row = Row::new(&title).note(&v.host).value(&time).value_ink(Ink::Muted);
        let row = row.leading(px(TILE) as u32).trailing(route_w(v.route) as u32);
        let open = PagePart::Open(k, key(&v.url, v.at_ms));
        let (lead, trail) = place(fb, g, c, (r, row), (Some(open), None));
        site_tile(fb, lead, &site_label::site(&v.url));
        route(fb, trail, v.route);
    }
}

/* The route's icon and name: Shield and Ok on an anonymising route, Globe
 * on the direct one, Shield on the proxy, both of those muted. */
fn route(fb: &mut PaintBuffer, t: Area, r: Route) {
    let ink = if matches!(r, Route::Mixnet | Route::Anyone) { Ink::Ok } else { Ink::Muted };
    let id = if r == Route::Direct { IconId::Globe } else { IconId::Shield };
    let side = scale().px(ICON).min(t.h);
    draw::icon(fb, id, t.x, t.y + (t.h - side) / 2, side, ink.colour(&roles()));
    let skip = side + scale().px(u32::from(SPACE.xs));
    let at = Area { x: t.x + skip, w: t.w.saturating_sub(skip), ..t };
    Label::new(r.name()).ink(ink).draw(fb, at);
}

/* The width the route's icon and name take. */
fn route_w(r: Route) -> i32 {
    let body = scale().font(TextRole::Body.px());
    px(ICON) + px(u32::from(SPACE.xs)) + text::width(r.name(), body) as i32
}
