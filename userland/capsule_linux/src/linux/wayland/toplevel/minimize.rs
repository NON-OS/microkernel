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

//! xdg_toplevel.set_minimized: the window minimised by the window manager,
//! as the frame's button does it, when the toplevel is the shown window's.
//! Its popups' grabs end first, as a focus-out ends them.

use nonos_app_skeleton::clients::wm::window_minimize;
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

pub fn minimize(guest: &mut Guest, toplevel: u32) {
    super::super::popup::ends::end_all(guest);
    if guest.scene.win.shown.and_then(|w| w.toplevel) != Some(toplevel) {
        return;
    }
    let (Some(wm), Some(window)) = (lookup_port(b"wm"), guest.scene.win.window) else { return };
    let rid = guest.scene.next_serial();
    let _ = window_minimize(wm, rid, window);
    // The window manager keeps the window; its layer comes off the
    // compositor so it is no longer drawn (present/hide.rs).
    super::super::present::hide::hide(&mut guest.scene);
}
