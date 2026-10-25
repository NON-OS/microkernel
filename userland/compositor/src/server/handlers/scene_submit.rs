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

use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::server::trusted_pids::trusted_pids;
use crate::state::attach_kernel::Kernel;
use crate::state::trusted_rule;
use crate::state::{scene_submit, Context};

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some(placed) = crate::requests::place::scene_submit(body) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    // The trusted band is the trusted sources' alone: a prompt is never
    // under a client's surface (state/trusted_rule.rs).
    if !trusted_rule::may_submit(sender_pid, placed.z, &trusted_pids()) {
        return respond::status(sender_pid, req, E_PERM, tx);
    }
    let Some(layer) = placed.layer(sender_pid, ctx.width, ctx.height) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    // A window that reopened its surface (a resize, for one) re-submits with a
    // new handle for the same owner and band. Only that band's layer is
    // replaced: a process holding layers in two bands keeps the other's. The
    // old handle's backing is gone, so drop it from the attach cache now
    // instead of letting it linger until the reaper runs. Otherwise a reused
    // handle number could serve a stale mapping and the compositor would paint from freed memory.
    let stale = ctx
        .scene
        .layers()
        .find(|l| {
            l.owner_pid == sender_pid && l.z == layer.z && l.surface_handle != layer.surface_handle
        })
        .map(|l| l.surface_handle);

    let Ok(repaint) = scene_submit::submit_layer(&mut ctx.scene, layer) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    if let Some(old) = stale {
        let _ = ctx.attach.forget(old, &mut Kernel);
    }
    for rect in repaint.into_iter().flatten() {
        ctx.damage.accumulate(rect);
    }
    respond::status(sender_pid, req, 0, tx)
}
