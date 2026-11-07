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

use crate::protocol::{Request, E_BUSY, E_INVAL, E_NOENT, E_PERM};
use crate::server::respond;
use crate::server::trusted_pids::trusted_pids;
use crate::state::attach_kernel::Kernel;
use crate::state::scene::Layer;
use crate::state::scene::Refused;
use crate::state::trusted_rule;
use crate::state::{scene_submit, Context};

use super::answers::answers_for;

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some(ask) = crate::requests::place_keyed::scene_submit_layer(body) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    // The trusted band is the trusted sources' alone, keyed or not.
    if !trusted_rule::may_submit(sender_pid, ask.placed.z, &trusted_pids()) {
        return respond::status(sender_pid, req, E_PERM, tx);
    }
    let Some(layer) = ask.placed.layer(sender_pid, ctx.width, ctx.height) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    // Only a surface the sender answers for is shown as its window.
    if !answers_for(ctx, sender_pid, layer.surface_handle) {
        return respond::status(sender_pid, req, E_PERM, tx);
    }
    let layer = Layer { window_id: ask.window_id, ..layer };
    let done = match scene_submit::submit_keyed_layer(&mut ctx.scene, layer, ask.replaces) {
        Ok(done) => done,
        Err(Refused::NoLayer) => return respond::status(sender_pid, req, E_NOENT, tx),
        Err(Refused::Full) => return respond::status(sender_pid, req, E_BUSY, tx),
    };
    // The surface the layer showed until now is gone with the resize that
    // replaced it; its mapping goes now, not when the reaper next runs, so a
    // reused handle number never serves the old one.
    if let Some(old) = done.released {
        let _ = ctx.attach.forget(old, &mut Kernel);
    }
    for rect in done.repaint.into_iter().flatten() {
        ctx.damage.accumulate(rect);
    }
    respond::status(sender_pid, req, 0, tx)
}
