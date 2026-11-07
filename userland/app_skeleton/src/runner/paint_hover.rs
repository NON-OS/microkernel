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

//! The lit frame control changed and nothing else asked for a repaint: the
//! window is drawn whole into a buffer the compositor released, so that
//! buffer holds the whole picture, and only the controls put out and lit
//! are presented (hover_part.rs). The app's own damage is left for its next
//! repaint, which presents it.

use nonos_toolkit::decorations::Rect;

use crate::app::App;
use crate::discover::Peers;
use crate::swap::Damage;

use super::boot::BootedApp;
use super::paint_frame::draw_whole;
use super::present_frame::{present, ACQUIRE_MS};
use super::request_id::next;

/// False when no buffer came free or the present was refused.
pub(super) fn paint_hover<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
    part: Rect,
) -> bool {
    let (state, maximized) = (booted.painted.2, booted.maximized);
    let b = &mut booted.binding;
    let Some((i, _)) = b.swap.acquire(ACQUIRE_MS) else { return false };
    draw_whole(&mut booted.app, &booted.manifest, b, i, (state, maximized));
    let x = part.x.min(b.width);
    let y = part.y.min(b.height);
    let (w, h) = (part.w.min(b.width - x), part.h.min(b.height - y));
    if w == 0 || h == 0 {
        return b.swap.cancel(i);
    }
    let d = Damage { x, y, width: w, height: h };
    present(b, peers.compositor, next(request_id), i, d)
}
