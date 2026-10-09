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
use crate::geometry::resize::resized;
use crate::protocol::{Request, E_INVAL, E_NOENT, WINDOW_RESIZE_REQ_LEN};
use crate::server::{full_screen_notify, respond};
use crate::state::Context;

/// The client resized its window: it keeps its origin and takes the size the
/// display leaves it there (geometry/resize.rs).
pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != WINDOW_RESIZE_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let u32_at = crate::server::handlers::u32_at::u32_at;
    let (Some(window_id), Some(w), Some(h)) = (u32_at(body, 0), u32_at(body, 8), u32_at(body, 12))
    else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    let (display_w, display_h) = (ctx.display_width, ctx.display_height);
    let was = full_screen_notify::covering(ctx, sender_pid, window_id);
    let Some(window) = ctx.windows.find_mut(sender_pid, window_id) else {
        let _ = respond::status(sender_pid, req, E_NOENT, tx);
        return;
    };
    if w == 0 || h == 0 {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    window.rect = resized(window.rect, w, h, display_w, display_h);
    // A window resized by its border is no longer full screen (app_skeleton
    // drops its maximised state on a resize too), so the dock comes back.
    window.full_screen = false;
    full_screen_notify::tell_if_changed(ctx, sender_pid, window_id, was);
    let _ = respond::status(sender_pid, req, 0, tx);
}
