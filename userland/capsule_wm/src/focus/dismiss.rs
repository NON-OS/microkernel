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

use crate::window::WindowTable;

/// The popups a press closes: every popup that closes on a press outside it
/// (a menu), when the pressed window is neither that popup nor one of the
/// popups over it. `pressed` is the project and id of the window pressed, or
/// None for a press on no window (the desk, the dock, the shell's menus).
/// Into `out` as (owner, id); how many. A popup over one of these goes with
/// it, so a menu's open submenu closes with the menu.
pub fn closed_by_press(
    table: &WindowTable,
    pressed: Option<(u32, u32)>,
    out: &mut [(u32, u32)],
) -> usize {
    let mut n = 0;
    for w in table.windows().filter(|w| w.is_popup() && w.grab) {
        let inside = pressed
            .is_some_and(|(pid, id)| pid == w.owner_pid && table.within(pid, w.window_id, id));
        if !inside && n < out.len() {
            out[n] = (w.owner_pid, w.window_id);
            n += 1;
        }
    }
    n
}
