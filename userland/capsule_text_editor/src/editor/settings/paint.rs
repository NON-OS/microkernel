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

//! The Settings screen: an opaque ground, the navigation rail, and the card
//! for the selected section, from its table. Every control is laid out afresh
//! in `c` on each paint.

use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::Controls;

use super::pick::Pick;
use super::rail::paint_rail;
use super::sect::section;
use super::sect_paint::paint_section;
use super::state::state;

pub(crate) fn paint_settings(fb: &mut PaintBuffer, c: &mut Controls<Pick>) {
    let (w, h) = (fb.width, fb.height);
    fb.fill_rect(0, 0, w, h, roles().bg.argb());
    c.clear();
    let nav = state().nav;
    paint_rail(fb, nav, c);
    if let Some(sec) = section(nav) {
        paint_section(fb, nav, sec, c);
    }
}
