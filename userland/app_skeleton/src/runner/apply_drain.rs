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

//! What one drain asked of the frame loop, in order: keyboard, place, resize,
//! move, then close, minimise or maximise, or restore and repaint.

use crate::app::App;
use crate::clients::{compositor, wm};
use crate::discover::Peers;

use super::boot::BootedApp;
use super::inbox::DrainResult;
use super::window_ops::{apply_move, apply_place, apply_resize, restore, toggle};
use super::{repaint::repaint, request_id::next, teardown::close};

/// True when the window closed and the frame loop must end.
pub(super) fn apply<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    result: DrainResult,
) -> bool {
    if let Some(has) = result.focused {
        booted.focused = has;
    }
    if let Some(p) = result.place {
        apply_place(booted, peers, request_id, p);
    }
    if let Some((rw, rh)) = result.resize_to {
        apply_resize(booted, peers, request_id, rw, rh);
    }
    if let Some((mx, my)) = result.move_to {
        apply_move(booted, peers, request_id, mx, my);
    }
    if result.close {
        // Once the user closes the window we must leave the frame loop no matter
        // what: close() releases and unmaps the surface, so continuing would
        // repaint into freed memory and fault every frame. Break unconditionally
        // and let the outer loop idle until the app is launched again.
        close(peers, booted.manifest.window_id, &booted.binding, request_id);
        return true;
    }
    if result.minimize {
        let _ = wm::window_minimize(peers.wm, next(request_id), booted.manifest.window_id);
        let _ = compositor::scene_remove(peers.compositor, next(request_id), 0);
        booted.minimized = true;
        return false;
    }
    if result.restore && booted.minimized {
        restore(booted, peers, request_id);
    }
    if result.maximize {
        booted.asks.full.person_chose();
        toggle(booted, peers, request_id);
        return false;
    }
    if result.repaint {
        repaint(booted, peers, request_id);
    }
    false
}
