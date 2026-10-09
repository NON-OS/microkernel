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

use crate::app::App;
use crate::clients::wm::WindowPlacement;
use crate::clients::{compositor, wm};
use crate::discover::Peers;

use super::boot::BootedApp;
use super::full_screen_ask::Turn;
use super::reopen::reopen;
use super::repaint::repaint;
use super::request_id::next;

/// The green button: full screen (chrome::full_screen, down to the bottom
/// edge, the dock hidden by the shell while it shows) and back to the rect
/// the window had. Every app goes through here, by green or by asking
/// (full_screen_ask.rs), and is drawn whole at its new size before it is
/// shown: the app paints to `fb.width` by `fb.height` (reopen.rs).
pub(super) fn toggle<A: App>(booted: &mut BootedApp<A>, peers: &Peers, request_id: &mut u32) {
    let (rect, full) = if booted.maximized {
        (booted.saved, false)
    } else if let Ok(di) = compositor::display_info(peers.compositor, next(request_id)) {
        booted.saved =
            (booted.binding.x, booted.binding.y, booted.binding.width, booted.binding.height);
        (super::chrome::full_screen(di.width, di.height), true)
    } else {
        repaint(booted, peers, request_id);
        return;
    };
    let (x, y, w, h) = rect;
    let placement = WindowPlacement { x, y, width: w, height: h };
    if reopen(booted, peers, request_id, placement, full) {
        let rid = next(request_id);
        let _ = wm::window_maximize(peers.wm, rid, booted.manifest.window_id, rect, full);
    } else {
        repaint(booted, peers, request_id);
    }
}

/// Follow the app's ask for full screen (`App::wants_full_screen`). A
/// minimised window waits: the ask is taken when it is shown again.
pub(super) fn follow_ask<A: App>(booted: &mut BootedApp<A>, peers: &Peers, request_id: &mut u32) {
    if booted.minimized {
        return;
    }
    let want = booted.app.wants_full_screen();
    let turn = booted.full_ask.step(want, booted.maximized);
    if turn != Turn::Stay {
        toggle(booted, peers, request_id);
    }
}
