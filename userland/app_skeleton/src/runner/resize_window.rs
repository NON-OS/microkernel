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
use super::reopen::reopen;
use super::request_id::next;

/// Reallocate the window's surface to `w`x`h` (keeping the origin), painted
/// at its new `fb.width`/`fb.height` before it is shown, and tell the window
/// manager. Same surface-reopen path maximize uses.
pub(super) fn apply_resize<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    w: u32,
    h: u32,
) {
    // Never below the size the app's layout holds at, and bounded by the
    // display right of and below the origin it keeps (min_size::room), so a
    // wild drag cannot allocate a surface larger than the screen.
    let (x, y) = (booted.binding.x, booted.binding.y);
    let bound = compositor::display_info(peers.compositor, next(request_id))
        .ok()
        .filter(|di| di.width > 0 && di.height > 0)
        .map(|di| super::min_size::room((di.width, di.height), (x, y)));
    let opened = (booted.manifest.width, booted.manifest.height);
    let (w, h) = super::min_size::settle((w, h), opened, bound);
    if w == booted.binding.width && h == booted.binding.height {
        return;
    }
    let placement = WindowPlacement { x, y, width: w, height: h };
    if reopen(booted, peers, request_id, placement, false) {
        let _ = wm::window_resize(peers.wm, next(request_id), booted.manifest.window_id, w, h);
        booted.full_ask.person_chose();
    }
}
