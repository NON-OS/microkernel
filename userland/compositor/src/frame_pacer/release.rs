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

//! After a frame: every buffer a present took off screen, and no layer
//! shows, is done with. Its release fence is signalled so its owner may
//! draw in it again. A buffer the kernel no longer has shared to us (its
//! owner exited or freed it) is let go all the same.

use crate::state::Context;

pub fn release(ctx: &mut Context) {
    if ctx.retiring.is_empty() {
        return;
    }
    let (scene, plane) = (&ctx.scene, ctx.direct.plane);
    // The plane may still scan a buffer no layer shows: it waits for the
    // flip that replaced it.
    let drawn = |h: u64| scene.layers().any(|l| l.surface_handle == h || l.content.handle == h);
    let shown = |h: u64| plane == Some(h) || drawn(h);
    ctx.retiring.release_unshown(shown, |h| {
        let _ = nonos_libc::mk_surface_fence_release(h);
    });
}
