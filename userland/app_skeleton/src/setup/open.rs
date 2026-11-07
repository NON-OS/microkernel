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

use crate::app::AppManifest;
use crate::discover::Peers;
use crate::swap::Swap;

use super::announce::announce;
use super::binding::{WindowBinding, BUFFERS};
use super::buffers::{open_buffers, release_buffers};
use super::submit_scene::APP_LAYER_Z;

pub fn open_window(
    peers: &Peers,
    manifest: &AppManifest,
    request_id: &mut u32,
) -> Result<WindowBinding, &'static str> {
    let (bufs, _, byte_len) = open_buffers(manifest.width, manifest.height)?;
    let pairs: [(u64, u64); BUFFERS] = core::array::from_fn(|i| (bufs[i].sid, bufs[i].handle));
    let Some(mut swap) = Swap::new(&pairs, APP_LAYER_Z) else {
        release_buffers(&bufs, byte_len);
        return Err("swapchain refused its buffers");
    };
    let placement = match announce(peers, manifest, bufs[0].handle, request_id) {
        Ok(p) => p,
        Err(e) => {
            release_buffers(&bufs, byte_len);
            return Err(e);
        }
    };
    // The layer shows the first buffer, blank, until the first frame is
    // presented in the other.
    swap.adopt(0);
    Ok(WindowBinding {
        bufs,
        swap,
        x: placement.x,
        y: placement.y,
        width: placement.width,
        height: placement.height,
        stride_words: manifest.width,
        byte_len,
    })
}
