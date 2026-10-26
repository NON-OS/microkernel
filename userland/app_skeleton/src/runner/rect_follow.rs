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

//! Following an app's ask for a rect (`rect_ask.rs`): the window is drawn
//! whole at the new rect before it is shown, as full screen does, and the
//! window manager is told where it now stands. A refusal leaves the window
//! where it was and is said once on the log.

use crate::app::App;
use crate::clients::wm::WindowPlacement;
use crate::clients::{compositor, wm};
use crate::discover::Peers;
use crate::log_line::{say, Line};

use super::boot::BootedApp;
use super::chrome::{dock_for, menubar_for};
use super::rect_ask::Move;
use super::reopen::reopen;
use super::request_id::next;

pub(super) fn follow_rect<A: App>(booted: &mut BootedApp<A>, peers: &Peers, request_id: &mut u32) {
    if booted.minimized || booted.maximized {
        return;
    }
    let want = booted.app.wants_rect();
    let b = &booted.binding;
    let now = (b.x, b.y, b.width, b.height);
    let mv = booted.asks.rect.step(want, now, || {
        let di = compositor::display_info(peers.compositor, next(request_id)).ok()?;
        let (bar, dock) = (menubar_for(di.width, di.height), dock_for(di.width, di.height));
        Some((0, bar, di.width, di.height.saturating_sub(bar + dock)))
    });
    let (x, y, width, height) = match mv {
        Move::Stay => return,
        Move::Place(r) | Move::Restore(r) => r,
    };
    if reopen(booted, peers, request_id, WindowPlacement { x, y, width, height }, false) {
        let id = booted.manifest.window_id;
        let _ = wm::window_move(peers.wm, next(request_id), id, x, y);
        let _ = wm::window_resize(peers.wm, next(request_id), id, width, height);
    } else {
        say(&Line::new(b"window").text(b": the rect asked for was refused; it keeps its place"));
    }
}
