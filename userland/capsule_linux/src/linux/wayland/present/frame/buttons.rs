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

//! The frame's three buttons. Close asks the guest to close, as a desktop
//! does (xdg_toplevel.close), and the guest decides; minimise is the window
//! manager's; maximise fills the display below the menubar, and F11 or the
//! guest brings it back, as a filled window has no frame.

use nonos_app_skeleton::clients::wm::window_minimize;
use nonos_app_skeleton::discover::lookup_port;

use crate::linux::guest::Guest;

use super::super::super::out::Event;
use super::super::super::scene::Scene;

/// xdg_toplevel.close.
const TOPLEVEL_CLOSE: u16 = 1;

pub fn close(guest: &mut Guest) {
    let Some(toplevel) = guest.scene.shown.and_then(|s| s.toplevel) else { return };
    Event::new(toplevel, TOPLEVEL_CLOSE).send(&mut guest.display.to_client);
}

pub fn minimize(scene: &mut Scene) {
    let (Some(wm), Some(id)) = (lookup_port(b"wm"), scene.window) else { return };
    let rid = scene.next_serial();
    let _ = window_minimize(wm, rid, id);
}

pub fn maximize(guest: &mut Guest) {
    super::super::super::toplevel::maximize(guest);
}
