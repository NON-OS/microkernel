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
use nonos_toolkit::decorations::{keep_far_edges, Placed};

use crate::app::App;
use crate::clients::wm::WindowPlacement;
use crate::clients::{compositor, wm};
use crate::discover::Peers;

use super::super::boot::BootedApp;
use super::super::reopen::reopen;
use super::super::request_id::next;

/// The window placed where a drag of its left or top edge left it: one new
/// surface at the new origin and size, then the WM told both at once
/// (OP_WINDOW_PLACE), so the window never shows moved but not resized. The
/// size is settled as a resize settles it, and an edge that did not move
/// stays where it was.
pub(in crate::runner) fn apply_place<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    p: Placed,
) {
    let b = &booted.binding;
    let bound = compositor::display_info(peers.compositor, next(request_id))
        .ok()
        .filter(|di| di.width > 0 && di.height > 0)
        .map(|di| super::super::min_size::room((di.width, di.height), (p.x, p.y)));
    let opened = (booted.manifest.width, booted.manifest.height);
    let (w, h) = super::super::min_size::settle((p.w, p.h), opened, bound);
    let base = Placed { x: b.x, y: b.y, w: b.width, h: b.height };
    let to = keep_far_edges(p, base, w, h);
    if to == base {
        return;
    }
    let placement = WindowPlacement { x: to.x, y: to.y, width: w, height: h };
    if reopen(booted, peers, request_id, placement, false) {
        let _ = wm::window_place(peers.wm, next(request_id), booted.manifest.window_id, placement);
        booted.asks.full.person_chose();
    }
}
