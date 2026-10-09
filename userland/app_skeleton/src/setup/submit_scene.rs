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

use crate::clients::compositor;
use crate::clients::wm::WindowPlacement;
use crate::discover::Peers;
use nonos_libc::mk_idle_ms;

use super::patience::SCENE_SUBMIT;
use super::request_id::bump;

const APP_LAYER_Z: u32 = 2;

pub(super) fn submit_scene(
    peers: &Peers,
    surface_handle: u64,
    request_id: &mut u32,
    placement: WindowPlacement,
) -> Result<(), &'static str> {
    let mut last = "compositor rejected scene_submit";
    for attempt in 0..SCENE_SUBMIT.attempts {
        let rid = bump(request_id);
        match compositor::scene_submit(
            peers.compositor,
            rid,
            surface_handle,
            placement.x,
            placement.y,
            placement.width,
            placement.height,
            APP_LAYER_Z,
        ) {
            Ok(()) => return Ok(()),
            Err(e) => last = e,
        }
        if attempt + 1 < SCENE_SUBMIT.attempts {
            mk_idle_ms(SCENE_SUBMIT.rest_ms);
        }
    }
    Err(last)
}
