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

//! The left rail: brand mark, the navigation destinations and Settings pinned
//! to the foot of the panel. `nav_row` and `settings_row` are the one geometry
//! source, so every hit-test lands on exactly the pill the painter drew.

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, text, Button, Kind, Separator, State};

use crate::ui::geometry::Rect;
use crate::ui::hover::Lit;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{cap_h, ITEM, S2, S3, S4, S5, S6, SECTION};
use crate::ui::state::{View, NAV};
use crate::ui::target::Target;

use super::chrome_geom::{nav_row, settings_row};

const GLYPHS: [Glyph; NAV.len()] =
    [Glyph::Home, Glyph::Grid, Glyph::Search, Glyph::Note, Glyph::Download];

/// An entry, its pill lit while `look` is selected: the page on show.
fn entry(fb: &mut PaintBuffer, icons: &Icons, r: Rect, (label, g): (&str, Glyph), look: State) {
    let (t, on) = (roles(), look.selected);
    let kind = if on { Kind::Tonal } else { Kind::Plain };
    Button::new("").kind(kind).state(look).draw(fb, r.tk());
    let ink = if on { t.text } else { t.text_2 };
    icons.centred(fb, Rect::new(r.x + S3, r.y, 30, r.h), 19, g, ink.argb());
    let tx = (r.x + S3 + 30 + S2).max(0) as u32;
    text::left(fb, tx, text::centre_top(r.tk().y, r.tk().h, ITEM), label, ink, ITEM);
}

pub fn sidebar(fb: &mut PaintBuffer, icons: &Icons, r: Rect, view: View, lit: &Lit) {
    let t = roles();
    draw::fill(fb, r.tk(), t.surface);
    draw::fill(fb, Rect::new(r.right() - 1, r.y, 1, r.h).tk(), t.border);

    let mark = Rect::new(r.x + S5, r.y + S6, 30, 30);
    draw::round(fb, mark.tk(), 9, t.accent_edge);
    draw::round(fb, mark.inset(5).tk(), 5, t.accent);
    let name_x = (mark.right() + S3).max(0) as u32;
    text::left(fb, name_x, mark.cy() - cap_h(SECTION), "Resonare", t.text, SECTION);

    for (i, (v, label)) in NAV.iter().enumerate() {
        let look = State { selected: *v == view, ..lit.of(Target::Nav(*v)) };
        entry(fb, icons, nav_row(r, i), (label, GLYPHS[i]), look);
    }

    let foot = settings_row(r);
    Separator::new().draw(fb, Rect::new(r.x + S5, foot.y - S4, r.w - S5 * 2, 1).tk());
    let look = State { selected: view == View::Settings, ..lit.of(Target::Nav(View::Settings)) };
    entry(fb, icons, foot, ("Settings", Glyph::Gear), look);
}
