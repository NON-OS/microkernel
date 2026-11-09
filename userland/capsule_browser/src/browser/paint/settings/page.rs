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
use nonos_toolkit::widgets::{Card, Row, SectionHeader, Separator};

use super::controls;
use super::{field, lines, shows::shows};
use crate::browser::paint::pages::extent::page_scroll;
use crate::browser::paint::pages::group_head::TITLE_PX;
use crate::browser::paint::tokens::palette;
use crate::browser::shell::Shell;
use crate::browser::state::State;
use crate::browser::ui::rect::{area, Rect};
use crate::browser::ui::settings_geom::{Control, Kind, SettingsGeom, BAND_H};

const PX: f32 = 17.0;
const SUB: &str = "Choices here hold for every tab in this window";

/// The settings page: a card per section under its header, each row's label
/// and the hairline between labelled rows, its text and its controls, laid
/// out afresh in the tab's settings controls; then the header band over
/// whatever has scrolled under it.
pub fn paint(state: &mut State, shell: &Shell, fb: &mut PaintBuffer) {
    fb.fill_rect(0, 0, fb.width, fb.height, palette().ground);
    let (w, h) = (fb.width as i32, fb.height as i32);
    let g = SettingsGeom::new(w, h, page_scroll(shell, w, h));
    let s = shows(state, shell);
    for it in g.items.iter() {
        let r = it.rect;
        match it.kind {
            Kind::Card => Card::new().draw(fb, area(r)),
            _ if r.y < 0 || r.y >= h => {}
            Kind::Head(t) => SectionHeader::new(t).draw(fb, area(r)),
            Kind::Label(t) => Row::new(t).draw(fb, area(r)),
            Kind::Rule => Separator::new().soft().draw(fb, area(rule(r))),
            Kind::Ctl(Control::Field(f)) => field::entry(&shell.settings, f).draw(fb, area(r)),
            Kind::Ctl(_) => {}
            Kind::Line(l) => lines::paint(fb, state, shell, l, r),
        }
    }
    controls::paint(fb, &g, h, &s, &mut state.ui.settings);
    fb.fill_rect(0, 0, fb.width, BAND_H.min(h).max(0) as u32, palette().ground);
    fb.text_ttf(g.frame.x, g.title_y, "Settings", palette().ink, TITLE_PX);
    fb.text_ttf(g.frame.x, g.sub_y, SUB, palette().muted, PX);
}

/* A rule's hairline, on the line between the rows it parts. */
fn rule(r: Rect) -> Rect {
    let hair = scale().hair() as i32;
    Rect::new(r.x, r.y - hair / 2, r.w, hair)
}
