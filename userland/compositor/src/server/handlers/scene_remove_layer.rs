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

use crate::protocol::{Request, E_INVAL, E_NOENT};
use crate::server::respond;
use crate::state::attach_kernel::Kernel;
use crate::state::{scene_remove, Context};

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some(ask) = crate::requests::place_keyed::scene_remove_layer(body) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    // The sender's own layer alone: no process names another's.
    let mine = |l: &&crate::state::scene::Layer| {
        l.owner_pid == sender_pid && l.z == ask.z && l.surface_handle == ask.handle
    };
    let content = ctx.scene.layers().find(mine).map_or(0, |l| l.content.handle);
    let Some(gone) = scene_remove::remove_layer(&mut ctx.scene, sender_pid, ask.z, ask.handle)
    else {
        return respond::status(sender_pid, req, E_NOENT, tx);
    };
    ctx.damage.accumulate(gone.rect);
    if let Some(handle) = gone.released {
        if !ctx.attach.forget(handle, &mut Kernel) {
            return respond::status(sender_pid, req, E_INVAL, tx);
        }
    }
    // No other layer draws a layer's content (scene/set_content.rs).
    if !ctx.attach.forget(content, &mut Kernel) {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    respond::status(sender_pid, req, 0, tx)
}
