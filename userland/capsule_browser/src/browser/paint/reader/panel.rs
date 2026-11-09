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
use nonos_toolkit::icons::IconId;
use nonos_toolkit::widgets::{Card, Controls, IconButton, Kind, Row, SectionHeader, Switch};

use super::controls::{self, area};
use crate::browser::shell::kept::ReaderPrefs;
use crate::browser::ui::reader_geom::ReaderGeom;
use crate::browser::ui::reader_hit::Control;

const TITLE: &str = "Reader Settings";
const LABELS: [&str; 3] = ["Appearance", "Text size", "Content width"];
const TIME: &str = "Show reading time";

/// The reader settings card, its controls laid out afresh in `c`: the title
/// and close button, then appearance, text size, content width and the
/// reading-time row, which works its switch from anywhere along it. A card
/// too narrow to be drawn places nothing.
pub fn paint(p: &ReaderPrefs, g: &ReaderGeom, fb: &mut PaintBuffer, c: &mut Controls<Control>) {
    c.clear();
    if !g.drawn() {
        return;
    }
    Card::new().title(TITLE).draw(fb, area(g.panel));
    let st = c.place(area(g.close), Control::Close);
    IconButton::new(IconId::Close).kind(Kind::Plain).state(st).draw(fb, area(g.close));
    for (r, s) in g.labels.iter().zip(LABELS) {
        SectionHeader::new(s).caption().draw(fb, area(*r));
    }
    controls::swatches(p, g, fb, c);
    controls::size(p, g, fb, c);
    controls::widths(p, g, fb, c);
    let at = area(g.time);
    let st = c.place(at, Control::Time);
    let switch = Switch::new(p.show_time);
    let row = Row::new(TIME).trailing(switch.size().0).state(st);
    row.draw(fb, at);
    switch.draw(fb, row.trailing_rect(at));
}
