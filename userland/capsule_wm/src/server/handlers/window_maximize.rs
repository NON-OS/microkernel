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

use crate::geometry::Rect;
use crate::protocol::{Request, E_INVAL, E_NOENT, WINDOW_MAXIMIZE_REQ_LEN};
use crate::server::{full_screen_notify, respond};
use crate::state::Context;
use crate::window::full_screen::maximize;
use crate::z_order::raise;

/// The body: `window_id u32, flags u32, x u32, y u32, w u32, h u32`. The flag
/// word was padding, sent as zero; bit 0 now makes the window full screen
/// (window/full_screen.rs), so an older client still restores and maximises
/// as before, with the dock kept.
pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != WINDOW_MAXIMIZE_REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let (Some(window_id), Some(flags), Some(x), Some(y), Some(w), Some(h)) = (
        super::u32_at::u32_at(body, 0),
        super::u32_at::u32_at(body, 4),
        super::u32_at::u32_at(body, 8),
        super::u32_at::u32_at(body, 12),
        super::u32_at::u32_at(body, 16),
        super::u32_at::u32_at(body, 20),
    ) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if w == 0 || h == 0 {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let (display_w, display_h) = (ctx.display_width, ctx.display_height);
    let was = full_screen_notify::covering(ctx, sender_pid, window_id);
    let Some(window) = ctx.windows.find_mut(sender_pid, window_id) else {
        let _ = respond::status(sender_pid, req, E_NOENT, tx);
        return;
    };
    maximize(window, Rect { x, y, width: w, height: h }, flags, display_w, display_h);
    if raise(&mut ctx.windows, &mut ctx.z, sender_pid, window_id) == Some(true) {
        crate::server::tell_compositor::lift(ctx, sender_pid);
    }
    full_screen_notify::tell_if_changed(ctx, sender_pid, window_id, was);
    let _ = respond::status(sender_pid, req, 0, tx);
}
