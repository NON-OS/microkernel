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
//! A drag of a maximized window's title bar out of full screen: the window
//! goes back to the size it had before, put under the pointer as the grip
//! says (drag/unmaximize.rs), drawn whole on its new surface before it is
//! shown, and the window manager told it is no longer full screen, so the
//! shell gives the dock back, as the green button does. The drag goes on
//! from the new place. A window that cannot take its old size stays
//! maximized and the drag ends there, so a surface that cannot be had is not
//! asked for again on every step of the pointer.

use crate::app::App;
use crate::clients::wm::{self, WindowPlacement};
use crate::discover::Peers;

use super::super::boot::BootedApp;
use super::super::drag::Grip;
use super::super::min_size::{MIN_H, MIN_W};
use super::super::reopen::reopen;
use super::super::request_id::next;

pub(in crate::runner) fn unmaximize<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    grip: Grip,
) {
    let (_, _, width, height) = booted.saved;
    if !booted.maximized || width < MIN_W || height < MIN_H {
        return;
    }
    let (x, y) = grip.restored_at(width, super::super::chrome::menubar_h());
    let placement = WindowPlacement { x, y, width, height };
    if !reopen(booted, peers, request_id, placement, false) {
        booted.drag.active = false;
        return;
    }
    // Leaving full screen by hand is the person's choice, as green is: an
    // app's later ask does not undo it (full_screen_ask.rs).
    booted.asks.full.person_chose();
    let (rid, rect) = (next(request_id), (x, y, width, height));
    let _ = wm::window_maximize(peers.wm, rid, booted.manifest.window_id, rect, false);
    booted.drag.rebase(grip.local, (x, y));
}
