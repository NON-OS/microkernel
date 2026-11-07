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
use crate::clients::{compositor, wm};
use crate::discover::Peers;

use super::super::boot::BootedApp;
use super::super::request_id::next;

const APP_LAYER_Z: u32 = 2;

pub(in crate::runner) fn apply_move<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    to_x: u32,
    to_y: u32,
) {
    let (w, h) = (booted.binding.width, booted.binding.height);
    // Asked once a drag, not once a step: a step is then one submit and one
    // notice to the window manager.
    let read = || {
        let di = compositor::display_info(peers.compositor, next(request_id)).ok()?;
        Some((di.width, di.height))
    };
    let Some((dw, dh)) = booted.drag.display_or(read) else { return };
    let nx = to_x.min(dw.saturating_sub(w));
    let ny = to_y.min(dh.saturating_sub(h));
    let (ox, oy) = (booted.binding.x, booted.binding.y);
    if nx == ox && ny == oy {
        return;
    }
    let submitted = compositor::scene_submit(
        peers.compositor,
        next(request_id),
        booted.binding.surface_handle(),
        nx,
        ny,
        w,
        h,
        APP_LAYER_Z,
    );
    // The submit damages the window's old place and its new one in the one
    // step; damage committed over the old place after it counts only on this
    // window's layers, now at the new place, and so adds nothing but a round
    // trip to every step of a drag (compositor state/scene_submit.rs).
    if submitted.is_err() {
        return;
    }
    let _ = wm::window_move(peers.wm, next(request_id), booted.manifest.window_id, nx, ny);
    booted.binding.x = nx;
    booted.binding.y = ny;
}
