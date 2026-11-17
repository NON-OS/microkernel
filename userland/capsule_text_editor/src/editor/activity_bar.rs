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

//! The left activity bar: stacked Explorer, Home and Settings navigation
//! items. The row of the screen on show is active, the Explorer row only
//! while its sidebar is open. Each row is laid out once a frame in the bar's
//! controls, so a press acts on the row drawn under it.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::rect::inset;
use nonos_toolkit::widgets::{draw, Controls, NavItem, Rect, Surface};

use super::screen::Screen;

/// The rows, top to bottom: the screen each one shows and its icon.
pub const ROWS: [(Screen, IconId); 3] = [
    (Screen::Editor, IconId::Files),
    (Screen::Home, IconId::Editor),
    (Screen::Settings, IconId::Settings),
];

/// Row `i`: an even share of `band`, `SPACE.xs` in from each side.
pub fn row(band: Rect, i: usize) -> Rect {
    let pitch = band.h / ROWS.len() as u32;
    let share = Rect { y: band.y.saturating_add(pitch.saturating_mul(i as u32)), h: pitch, ..band };
    inset(share, scale().px(u32::from(SPACE.xs)))
}

/// Fills `column`, lays the rows of `band` out in `c` and paints them.
pub fn paint_rail(
    fb: &mut impl Surface,
    column: Rect,
    band: Rect,
    on_show: (Screen, bool),
    c: &mut Controls<Screen>,
) {
    let (screen, sidebar_open) = on_show;
    draw::fill(fb, column, roles().sunken);
    c.clear();
    for (i, (target, icon)) in ROWS.into_iter().enumerate() {
        let at = row(band, i);
        let active = target == screen && (target != Screen::Editor || sidebar_open);
        NavItem::new("").icon(icon).active(active).state(c.place(at, target)).draw(fb, at);
    }
}
