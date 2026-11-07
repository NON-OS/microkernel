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

use nonos_toolkit::decorations::FrameState;

use crate::app::App;
use crate::discover::Peers;

use super::boot::BootedApp;
use super::hide_steps::waits_for_frame;
use super::paint_frame::paint_changed;

/// The window painted whole, if its last frame was not shown: the first
/// one, or one that found no buffer free or was refused. It is presented
/// as the rows it changed from the frame on screen (paint_changed), since
/// a frame not shown left the shown buffer as it was: presented whole, a
/// window whose composite outlasted the wait for a buffer had every later
/// key composed whole too, each one as slow (a maximized Terminal under
/// TCG, 1,962,240 px a key). A window on the dock is not painted: it is
/// painted whole as it is shown again.
pub(super) fn ensure_primed<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
) -> bool {
    if !waits_for_frame(booted.primed, booted.minimized) {
        return true;
    }
    let state = FrameState { hover: booted.drag.hover, focused: booted.focused };
    let BootedApp { app, manifest, binding, maximized, .. } = booted;
    booted.primed = paint_changed(app, manifest, binding, (state, *maximized), peers, request_id);
    booted.primed
}
