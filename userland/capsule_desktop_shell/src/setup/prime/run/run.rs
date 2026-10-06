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

use nonos_libc::mk_surface_release;

use crate::render::paint_chrome;
use crate::setup::prime::register::{CHROME_Z, DESK_Z};
use crate::setup::prime::{open_chrome_windows, overlay, peers};
use crate::state::Context;

pub fn run() -> Result<Context, &'static str> {
    let peers = peers::resolve()?;
    super::healthcheck_peers::healthcheck_peers(&peers)?;
    let wallpaper_policy_sent = crate::server::wallpaper_policy::send(peers.wallpaper_port);
    let overlay = overlay::allocate(peers.compositor_port, 1)?;
    let desk = match overlay::allocate(peers.compositor_port, 1) {
        Ok(desk) => desk,
        Err(e) => {
            overlay::free(&overlay);
            return Err(e);
        }
    };
    // Every surface registered so far. A step that fails after the backings
    // are mapped gives both back before setup goes round again: each round
    // used to map two more screen sized backings and leave the old ones, and
    // their surfaces, behind.
    let mut held: [Option<u64>; 2] = [None, None];
    match finish(&peers, &overlay, &desk, wallpaper_policy_sent, &mut held) {
        Ok(ctx) => Ok(ctx),
        Err(e) => {
            // The compositor must stop reading a surface before its memory
            // goes: release each handle, then unmap.
            for handle in held.iter().flatten() {
                let _ = mk_surface_release(*handle);
            }
            overlay::free(&overlay);
            overlay::free(&desk);
            Err(e)
        }
    }
}

fn finish(
    peers: &peers::Peers,
    overlay: &overlay::Overlay,
    desk: &overlay::Overlay,
    wallpaper_policy_sent: bool,
    held: &mut [Option<u64>; 2],
) -> Result<Context, &'static str> {
    let mut ctx = super::build_context::build_context(peers, overlay, desk, wallpaper_policy_sent);
    crate::render::ui_font::set_scale(ctx.scale);
    paint_chrome(&mut ctx);
    // The desk first, so the compositor never shows the chrome without the
    // desktop under it.
    held[0] = Some(super::register_overlay::register_overlay(&mut ctx, desk, DESK_Z)?);
    held[1] = Some(super::register_overlay::register_overlay(&mut ctx, overlay, CHROME_Z)?);
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
