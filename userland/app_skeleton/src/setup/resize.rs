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

use crate::clients::wm::WindowPlacement;
use crate::discover::Peers;
use crate::swap::Swap;

use super::binding::{WindowBinding, BUFFERS};
use super::buffers::{open_buffers, release_buffers};
use super::submit_scene::{submit_scene, APP_LAYER_Z};

/// New buffers at `placement`'s size. `paint` draws the first whole before
/// the layer is submitted with it, so the window is never shown half
/// drawn; the old buffers go once the compositor has the new.
pub fn reopen_surface(
    peers: &Peers,
    old: &WindowBinding,
    placement: WindowPlacement,
    request_id: &mut u32,
    paint: impl FnOnce(&mut WindowBinding),
) -> Result<WindowBinding, &'static str> {
    let WindowPlacement { x, y, width: w, height: h } = placement;
    let (bufs, _, byte_len) = open_buffers(w, h)?;
    let pairs: [(u64, u64); BUFFERS] = core::array::from_fn(|i| (bufs[i].sid, bufs[i].handle));
    let Some(swap) = Swap::new(&pairs, APP_LAYER_Z) else {
        release_buffers(&bufs, byte_len);
        return Err("swapchain refused its buffers");
    };
    let mut binding =
        WindowBinding { bufs, swap, x, y, width: w, height: h, stride_words: w, byte_len };
    paint(&mut binding);
    if let Err(e) = submit_scene(peers, bufs[0].handle, request_id, placement) {
        release_buffers(&bufs, byte_len);
        return Err(e);
    }
    binding.swap.adopt(0);
    release_buffers(&old.bufs, old.byte_len);
    Ok(binding)
}
