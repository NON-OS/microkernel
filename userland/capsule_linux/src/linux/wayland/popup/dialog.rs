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

//! A toplevel with a parent (xdg_toplevel.set_parent): a dialog, a file
//! chooser, a message box. Shown as its own window it took the place of the
//! app's one window, at its own size; it is laid instead on that window, a
//! grabbing layer centred on it, as a popup is (layer.rs), so a maximised
//! app keeps its size and the dialog takes the keys and the pointer until
//! it goes. With no window shown yet it is shown as a window, as before.
//! Its first buffer lays it (dialog_lay.rs).

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::input::enter::keyboard_on;
use super::super::ops::ev;
use super::super::out::Event;
use super::super::subsurface::compose::repaint;

/// set_parent on `toplevel`: a dialog when the parent is set, else not.
pub fn set_parent(guest: &mut Guest, toplevel: u32, args: &mut Args<'_>) {
    let Some(parent) = args.u32() else { return };
    gone(guest, toplevel);
    if parent == 0 {
        return;
    }
    guest.scene.popups.dialogs.push(toplevel);
    // A size of its own choosing, not the window's.
    let xdg =
        guest.scene.surfaces.iter().find(|s| s.toplevel == Some(toplevel)).and_then(|s| s.xdg);
    let serial = guest.scene.next_serial();
    Event::new(toplevel, ev::TOPLEVEL_CONFIGURE)
        .u32(0)
        .u32(0)
        .u32(0)
        .send(&mut guest.display.to_client);
    if let Some(xdg) = xdg {
        Event::new(xdg, ev::XDG_SURFACE_CONFIGURE).u32(serial).send(&mut guest.display.to_client);
    }
}

/// Dialog `toplevel` taken off the window, and the keys back to what is on top.
pub fn gone(guest: &mut Guest, toplevel: u32) {
    guest.scene.popups.dialogs.retain(|t| *t != toplevel);
    let list = &mut guest.scene.popups.list;
    let Some(at) = list.iter().position(|p| p.id == toplevel) else { return };
    let p = list.remove(at);
    super::layer::take(&mut guest.scene, p.surface);
    if let Some(keyboard) = guest.scene.keyboard {
        let _ = keyboard_on(guest, keyboard);
    }
    repaint(guest);
}
