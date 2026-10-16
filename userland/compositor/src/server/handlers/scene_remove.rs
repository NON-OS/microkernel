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

use crate::protocol::{Request, E_INVAL, SCENE_REMOVE_REQ_LEN};
use crate::server::respond;
use crate::state::attach_kernel::Kernel;
use crate::state::{scene_remove, Context};

pub const BAND_ONLY: u32 = 1;

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if body.len() != SCENE_REMOVE_REQ_LEN {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    if sender_pid == 0 {
        return respond::status(sender_pid, req, E_INVAL, tx);
    }
    // Body: z u32, flags u32. BAND_ONLY drops the sender's layer in band z
    // alone (the shell moving its chrome in or out of the trusted band);
    // without it every layer the sender holds goes, as before.
    let (z, flags) = (super::u32_at(body, 0).unwrap_or(0), super::u32_at(body, 4).unwrap_or(0));
    if flags & BAND_ONLY != 0 {
        return super::scene_remove_band::handle(ctx, sender_pid, z, req, tx);
    }
    let owner_pid = sender_pid;
    let mut gone = [0u64; 32];
    let mut n = 0;
    for layer in ctx.scene.layers().filter(|l| l.owner_pid == owner_pid) {
        if n < gone.len() {
            gone[n] = layer.surface_handle;
            n += 1;
        }
    }
    if let Some(rect) = scene_remove::remove_by_pid(&mut ctx.scene, owner_pid) {
        ctx.damage.accumulate(rect);
    }
    let mut released = true;
    for handle in gone.iter().take(n) {
        released &= ctx.attach.forget(*handle, &mut Kernel);
    }
    respond::status(sender_pid, req, if released { 0 } else { E_INVAL }, tx)
}
