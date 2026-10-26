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
//! A frame just drawn, shown: each surface that flips (state/ctx/flip.rs)
//! has the backing the frame went to submitted in place of the shown one,
//! which the compositor takes in one step. A refused submit leaves the shown
//! frame on screen; the next repaint draws and submits again.

use crate::compositor_client::push_scene_submit;
use crate::setup::prime::bands::DESK_Z;
use crate::state::trusted_rule::{CHROME_Z, TRUSTED_Z};
use crate::state::Context;

pub(super) fn show(ctx: &mut Context) {
    if ctx.desk.flips() && submit(ctx, ctx.desk.next_handle(), DESK_Z) {
        ctx.desk.flipped();
    }
    let chrome_z = if ctx.trusted.up { TRUSTED_Z } else { CHROME_Z };
    if ctx.chrome.flips() && submit(ctx, ctx.chrome.next_handle(), chrome_z) {
        ctx.chrome.flipped();
    }
}

fn submit(ctx: &mut Context, handle: u64, z: u32) -> bool {
    let rid = ctx.issue_request_id();
    let rect = (0, 0, ctx.width, ctx.height);
    push_scene_submit(ctx.ports.compositor_port, rid, handle, rect, z).is_ok()
}
