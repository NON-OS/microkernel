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

//! What a toplevel may fill (xdg_toplevel 4, configure_bounds), and what
//! this desktop can do with it (5, wm_capabilities). The bounds are a
//! maximised window's content, below the menubar and inside the NØNOS frame,
//! in the logical pixels the display's scale makes; 0 by 0, which says they
//! are not known, without a display.

use crate::linux::guest::Guest;

use super::super::out::Event;
use super::super::present::frame::now::inset;
use super::super::present::scale_math::{display, to_logical};
use super::super::present::scaled::quarters;
use super::ask::fill;

const TOPLEVEL_WM_CAPABILITIES: u16 = 3;
/// What the person can do to the window: maximise and full screen (the
/// frame's button, F11), minimise (the frame's button). No window menu.
const CAPABILITIES: [u32; 3] = [2, 3, 4];

/// The bounds `toplevel` is told, when it was made at 4 or later.
pub fn of(guest: &Guest, toplevel: u32) -> Option<(u32, u32)> {
    guest.objects.at_least(toplevel, 4).then(|| area(guest))
}

/// What the display holds of a window's content, logical; 0 by 0 unknown.
pub fn area(guest: &Guest) -> (u32, u32) {
    let Some(rect) = fill() else { return (0, 0) };
    let (_, _, w, h) = inset(&guest.scene, rect);
    let d = display(quarters());
    (to_logical(w as i32, d) as u32, to_logical(h as i32, d) as u32)
}

/// Before the first configure, a toplevel made at 5 or later.
pub fn capabilities(guest: &mut Guest, toplevel: u32) {
    if guest.objects.at_least(toplevel, 5) {
        let ev = Event::new(toplevel, TOPLEVEL_WM_CAPABILITIES).words(&CAPABILITIES);
        ev.send(&mut guest.display.to_client);
    }
}
