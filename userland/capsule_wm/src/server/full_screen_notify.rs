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

//! Tell lifecycle subscribers when a window starts or stops covering the
//! dock's band (window/full_screen.rs). The desktop shell hides its dock
//! while any window does; this table is the one place that knows, since a
//! window is made full screen, minimised and restored here, by its client
//! and by the shell's dock alike.

use crate::protocol::NOTIFY_KIND_FULL_SCREEN;
use crate::state::Context;
use crate::window::full_screen::covers_dock_at;

/// Whether `pid`'s window `window_id` covers the dock's band, read before a
/// change so `tell_if_changed` can compare.
pub fn covering(ctx: &Context, pid: u32, window_id: u32) -> bool {
    covers_dock_at(&ctx.windows, pid, window_id)
}

/// Broadcast the window's state when it differs from `was`. Only a change is
/// sent: the dock restores windows that were never minimised, and a resize
/// of a window that was never full screen changes nothing for the shell.
pub fn tell_if_changed(ctx: &mut Context, pid: u32, window_id: u32, was: bool) {
    let now = covering(ctx, pid, window_id);
    if now == was {
        return;
    }
    crate::server::notify_fanout::broadcast(
        ctx,
        NOTIFY_KIND_FULL_SCREEN,
        pid,
        window_id,
        now as u32,
        0,
    );
}
