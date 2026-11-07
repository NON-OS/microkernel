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

//! The window's toplevel told its state again when only `activated` changed:
//! the window gained or lost the keyboard (input/focus.rs). Its mode and
//! size are what it has or is on its way to (fit.rs `again`), and a window in
//! its own size is told that size (current.rs).

use crate::linux::guest::Guest;

use super::super::configure::configure;
use super::super::fit::{Configure, Mode};
use super::ask::logical;

/// The shown window's toplevel, configured once more as it is now.
pub fn again(guest: &mut Guest) {
    let activated = guest.scene.win.focused;
    again_as(guest, activated);
}

/// The same, `activated` or not.
fn again_as(guest: &mut Guest, activated: bool) {
    let Some(toplevel) = guest.scene.win.shown.and_then(|w| w.toplevel) else { return };
    let owner = guest.scene.surfaces.iter().find(|s| s.toplevel == Some(toplevel));
    // One still owed its first configure is told this in that one.
    let Some(xdg) = owner.filter(|s| !s.initial).and_then(|s| s.xdg) else { return };
    let serial = guest.scene.next_serial();
    // Another toplevel (a dialog) has no mode of its own here.
    let told = match guest.scene.win.fit_of == Some(toplevel) {
        true => logical(guest.scene.win.fit.again(serial), guest.scene.win.frame.ratio),
        false => Configure { serial, mode: Mode::Normal, rect: None },
    };
    // In its own size: that size, said again (current.rs).
    let size = super::current::current(guest, toplevel);
    let told = match (told.rect, size) {
        (None, Some((w, h))) => Configure { rect: Some((0, 0, w, h)), ..told },
        _ => told,
    };
    let how = (activated, super::bounds::of(guest, toplevel));
    configure(toplevel, xdg, &told, how, &mut guest.display.to_client);
}
