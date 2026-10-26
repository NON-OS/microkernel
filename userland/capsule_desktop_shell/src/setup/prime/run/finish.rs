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

//! The rest of setup once both surfaces are mapped: the context built and
//! painted, the desk and chrome registered (each handle noted as it is had,
//! so a failure can give them back), the first frame, the windows, the
//! subscriptions, and the chime.

use super::build_context::build_context;
use crate::render::paint_chrome;
use crate::setup::prime::bands::{CHROME_Z, DESK_Z};
use crate::setup::prime::{open_chrome_windows, overlay, peers};
use crate::state::ctx::flip::Flip;
use crate::state::Context;

pub(super) fn finish(
    peers: &peers::Peers,
    [chrome, desk]: [&[overlay::Overlay; 2]; 2],
    wallpaper_policy_sent: bool,
    held: &mut [Option<u64>; 4],
) -> Result<Context, &'static str> {
    let mut ctx = build_context(peers, &chrome[0], &desk[0], wallpaper_policy_sent);
    crate::render::ui_font::set_scale(ctx.scale);
    paint_chrome(&mut ctx);
    // Each surface's second backing first, then the painted one, so the one
    // submitted last and shown is the painted one; the desk before the
    // chrome, so the compositor never shows the chrome without the desktop.
    let order =
        [(&desk[1], DESK_Z), (&desk[0], DESK_Z), (&chrome[1], CHROME_Z), (&chrome[0], CHROME_Z)];
    for (slot, (o, z)) in held.iter_mut().zip(order) {
        *slot = Some(super::register_overlay::register_overlay(&mut ctx, o, z)?);
    }
    let h = held.map(|h| h.unwrap_or(0));
    ctx.desk =
        Flip { va: [desk[0].backing_va, desk[1].backing_va], handle: [h[1], h[0]], shown: 0 };
    let chrome_va = [chrome[0].backing_va, chrome[1].backing_va];
    ctx.chrome = Flip { va: chrome_va, handle: [h[3], h[2]], shown: 0 };
    // The first frame is asked for once; the runner's first repaint asks
    // again, so a busy compositor here does not send setup round.
    super::commit_overlay::commit_overlay(&mut ctx);
    open_chrome_windows::open_chrome_windows(&mut ctx)?;
    super::subscribe_wm::subscribe_wm(&mut ctx, peers.wm_port);
    super::subscribe_input::subscribe_input(&mut ctx, peers.input_router_port);
    // Last, and only where a context is returned: the desktop is committed,
    // painted and listening, which is what finished booting looks like.
    crate::sound::chime();
    Ok(ctx)
}
