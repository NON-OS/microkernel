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

//! The wallpaper's place in the compositor's scene, asked for until the
//! compositor says it has it.
//!
//! The compositor reads its requests between frames, and a frame it composes
//! whole (at boot, and every few seconds to heal) can outlast a call's
//! budget; the scene submit used to get 16 ms, and one that timed out sent
//! setup back to the start ("unanswered compositor caller=wallpaper", err
//! ETIMEDOUT). Now setup asks once with a frame's worth of time, and the
//! service loop asks again, at a growing interval, until an answer comes,
//! while it serves. Asking again is safe: the compositor keeps one layer per
//! owner and band, so the same surface submitted twice is the same layer.

use nonos_libc::mk_debug;

use crate::compositor_client::push_scene_submit;
use crate::state::Context;

/// The band under every window.
const BOTTOM_Z: u32 = 0;
/// The try setup makes, before the service answers anyone: time for the
/// compositor to finish a whole frame first.
pub const SETUP_MS: u64 = 250;
/// A later try, from the service loop, whose own callers wait meanwhile.
const RETRY_MS: u64 = 100;

/// Ask the compositor to put the wallpaper in its scene. True once it said so.
pub fn submit(ctx: &mut Context, timeout_ms: u64) -> bool {
    let rid = ctx.issue_request_id();
    let (port, handle, width, height) =
        (ctx.compositor_port, ctx.surface_handle, ctx.width, ctx.height);
    push_scene_submit(port, rid, handle, width, height, BOTTOM_Z, timeout_ms).is_ok()
}

/// Setup's try. Unanswered, the service loop takes over.
pub fn first_submit(ctx: &mut Context) {
    ctx.registered = submit(ctx, SETUP_MS);
    if !ctx.registered {
        ctx.register_backoff.failed();
        say(b"[WALLPAPER] the compositor has not taken the wallpaper yet; asking again\n");
    }
}

/// Each turn of the service loop: ask again while the compositor has not
/// answered, when the backoff says so.
pub fn keep_registered(ctx: &mut Context) {
    if ctx.registered || !ctx.register_backoff.due() {
        return;
    }
    if submit(ctx, RETRY_MS) {
        ctx.registered = true;
        ctx.register_backoff.reset();
        say(b"[WALLPAPER] the compositor has taken the wallpaper\n");
    } else {
        ctx.register_backoff.failed();
    }
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
