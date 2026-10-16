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

use crate::protocol::{Request, E_INVAL};
use crate::server::respond;
use crate::state::damage::Rect;
use crate::state::damage_bound as bound;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    // Only an empty rectangle or one off the canvas is refused; one past
    // an edge is cut to it (requests/area.rs).
    let area = crate::requests::area::damage_commit(body);
    let Some(a) = area.and_then(|a| a.clip(ctx.width, ctx.height)) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let (x, y, width, height) = (a.x, a.y, a.width, a.height);
    // Only where the sender's own layers are (state/damage_bound.rs). Damage
    // past them is answered 0, not refused: a window that damages the place
    // it just left was already repainted there by its move.
    let r = Rect { x, y, width, height };
    for part in bound::by_owner(r, ctx.scene.layers(), sender_pid) {
        ctx.damage.accumulate(part);
    }
    respond::status(sender_pid, req, 0, tx)
}
