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

//! A toplevel's first configure. xdg-shell has the client make its initial
//! commit, with no buffer, once it has said what it wants of the window
//! (its title, its size limits, full screen or maximised); the configure
//! answers that commit. A mode it asked for before then is told in it, and
//! otherwise the size it is offered (size.rs).

use crate::linux::guest::Guest;

use super::super::configure::configure;
use super::super::fit::{Configure, Mode};
use super::ask::ask;

/// A commit on `surface`: its toplevel's first configure, when still owed.
pub fn first(guest: &mut Guest, surface: u32) {
    let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface && s.initial) else {
        return;
    };
    s.initial = false;
    let (wants, parent, limits) = (s.wants.take(), s.parent, s.limits);
    let (Some(toplevel), Some(xdg)) = (s.toplevel, s.xdg) else { return };
    super::super::decoration::undecorated(guest, surface, toplevel);
    super::bounds::capabilities(guest, toplevel);
    // While another window is shown the mode is asked once this one is
    // (ask.rs held), so it is configured at its own size now: with no
    // configure it would never draw, and never be shown.
    let other = guest.scene.win.shown.is_some_and(|w| w.toplevel != Some(toplevel));
    match wants {
        Some(mode) if mode != Mode::Normal && !other => ask(guest, toplevel, mode),
        _ => {
            if let Some(s) = guest.scene.surfaces.iter_mut().find(|s| s.id == surface) {
                s.wants = wants.filter(|&m| m != Mode::Normal);
            }
            let (w, h) = super::size::offered(parent, limits, super::bounds::area(guest));
            let serial = guest.scene.next_serial();
            let told = Configure { serial, mode: Mode::Normal, rect: Some((0, 0, w, h)) };
            let how = (guest.scene.win.focused, super::bounds::of(guest, toplevel));
            configure(toplevel, xdg, &told, how, &mut guest.display.to_client);
        }
    }
}
