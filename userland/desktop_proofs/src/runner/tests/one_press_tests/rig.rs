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

//! Desks of two windows, with and without the dock.

use super::super::super::desk::Desk;

pub(super) const TERMINAL: u32 = 10;

pub(super) const EDITOR: u32 = 20;

/// The terminal's tab strip with one tab: a pill, the new-tab chip and three
/// tool buttons (capsule_terminal's nominal_w).
const TABS_W: u32 = 160 + 28 + 3 * 30;

/// The editor opened second, over the terminal's lower half, and focused.
/// The terminal's title bar runs from y 70 to 110 on screen, its tabs at the
/// right of it from x 460 to 738 (y 77 to 103).
pub(super) fn desk() -> Desk {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (60, 60, 700, 400), TABS_W);
    d.open(EDITOR, (300, 150, 900, 520), 0);
    assert_eq!(d.focused(), EDITOR);
    assert_eq!(d.shown_at(400, 300), EDITOR, "the editor covers the terminal's lower half");
    d.assert_consistent("two windows open");
    d
}

pub(super) const SHELL: u32 = 5;

pub(super) const DOCK_WINDOW_ID: u32 = 0x5442_4152;

/// The shell on a 1280 by 800 screen: its dock 64 high, 16 above the bottom
/// edge; and the editor as fit_to_display sizes and centres it there, 663 high
/// from y 91, over the dock's top 34 rows.
pub(super) fn desk_with_dock() -> Desk {
    let mut d = Desk::new(1280, 800);
    d.open_shell(SHELL, (440, 720, 400, 64));
    d.open(EDITOR, (77, 91, 1126, 663), 0);
    d
}
