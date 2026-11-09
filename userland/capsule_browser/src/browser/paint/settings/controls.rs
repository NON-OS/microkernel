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
use nonos_toolkit::widgets::{Button, Controls, IconButton, Kind as Weight, State};

use super::caption::caption;
use super::look::{look, Look, Shows};
use crate::browser::ui::rect::{area, Rect};
use crate::browser::ui::settings_geom::{Control, Kind, SettingsGeom, BAND_H};

/// Lays out every control of the page `h` tall in `c` and draws its
/// buttons. A control's box is the part of it below the header band, which
/// covers the rest; one wholly under the band or below the page takes
/// nothing. A choice in force is Primary, Remove all bookmarks is Danger and
/// armed while it waits for its second click, and a button with nothing to
/// act on is disabled. The text size steps are a minus and a plus icon
/// button. The fields are placed here and drawn by the page.
pub fn paint(fb: &mut PaintBuffer, g: &SettingsGeom, h: i32, s: &Shows, c: &mut Controls<Control>) {
    c.clear();
    for it in g.items.iter() {
        let Kind::Ctl(ctl) = it.kind else { continue };
        let r = it.rect;
        let top = r.y.max(BAND_H);
        if r.y < 0 || r.y >= h || top >= r.bottom() {
            continue;
        }
        let at = area(Rect::new(r.x, top, r.w, r.bottom() - top));
        let l = look(s, ctl);
        let st = if l == Look::Dim { c.place_disabled(at, ctl) } else { c.place(at, ctl) };
        if matches!(ctl, Control::Field(_)) {
            continue;
        }
        let weight = match l {
            Look::On => Weight::Primary,
            Look::Danger | Look::Armed => Weight::Danger,
            _ => Weight::Secondary,
        };
        let st = State { selected: l == Look::Armed, ..st };
        match step(ctl) {
            Some(id) => IconButton::new(id).kind(weight).state(st).draw(fb, area(r)),
            None => Button::new(&caption(s, ctl)).kind(weight).state(st).draw(fb, area(r)),
        }
    }
}

/* The icon a text size step shows, or None for any other control. */
fn step(c: Control) -> Option<IconId> {
    match c {
        Control::Smaller => Some(IconId::Minus),
        Control::Larger => Some(IconId::Plus),
        _ => None,
    }
}
