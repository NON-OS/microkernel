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

use super::finish::finish;
use crate::setup::prime::{overlay, peers};
use crate::state::Context;
use nonos_libc::mk_surface_release;

pub fn run() -> Result<Context, &'static str> {
    let peers = peers::resolve()?;
    crate::bench::mark(b"peers");
    super::healthcheck_peers::healthcheck_peers(&peers)?;
    crate::bench::mark(b"peers_healthy");
    let wallpaper_policy_sent = crate::setup::wallpaper_policy::send(peers.wallpaper_port);
    let [c0, c1, d0, d1] = allocate_four(peers.compositor_port)?;
    crate::bench::mark(b"backings");
    let (chrome, desk) = ([c0, c1], [d0, d1]);
    // Every surface registered so far. A step that fails after the backings
    // are mapped gives them all back before setup goes round again: each
    // round used to map more screen sized backings and leave the old ones,
    // and their surfaces, behind.
    let mut held: [Option<u64>; 4] = [None; 4];
    match finish(&peers, [&chrome, &desk], wallpaper_policy_sent, &mut held) {
        Ok(ctx) => Ok(ctx),
        Err(e) => {
            // The compositor must stop reading a surface before its memory
            // goes: release each handle, then unmap.
            for handle in held.iter().flatten() {
                let _ = mk_surface_release(*handle);
            }
            chrome.iter().chain(&desk).for_each(overlay::free);
            Err(e)
        }
    }
}

/// The chrome's two backings and the desk's, each the size of the display;
/// any mapped before one fails is given back.
fn allocate_four(port: u32) -> Result<[overlay::Overlay; 4], &'static str> {
    let mut got = alloc::vec::Vec::with_capacity(4);
    for _ in 0..4 {
        match overlay::allocate(port, 1) {
            Ok(o) => got.push(o),
            Err(e) => {
                got.iter().for_each(overlay::free);
                return Err(e);
            }
        }
    }
    got.try_into().map_err(|_| "overlay count")
}
