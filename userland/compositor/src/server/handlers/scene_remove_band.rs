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

//! SCENE_REMOVE with BAND_ONLY: the sender's layer in one band dropped and
//! its rectangle repainted; its surface's mapping forgotten only when no
//! other layer still shows that surface (the shell moves one surface between
//! two bands, briefly in both).

use crate::protocol::{Request, E_INVAL};
use crate::server::respond;
use crate::state::attach_kernel::Kernel;
use crate::state::scene_submit::rect_of;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    z: u32,
    req: &Request,
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some(gone) = ctx.scene.drop_band(sender, z) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    ctx.damage.accumulate(rect_of(&gone));
    let shown = ctx.scene.layers().any(|l| l.surface_handle == gone.surface_handle);
    if !shown && !ctx.attach.forget(gone.surface_handle, &mut Kernel) {
        return respond::status(sender, req, E_INVAL, tx);
    }
    respond::status(sender, req, 0, tx)
}
