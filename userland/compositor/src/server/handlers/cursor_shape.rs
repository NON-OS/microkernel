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
//! A window's owner sets the pointer's shape over its own surface: the
//! resize arrows over its edges, the arrow elsewhere (app_skeleton asks as
//! the pointer crosses them). A shape changed under the pointer is drawn
//! at once.

use crate::protocol::{Request, E_INVAL, E_NOENT};
use crate::server::respond;
use crate::state::cursor_pick::reach;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some((handle, code)) = crate::requests::pointer::cursor_shape(body) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    let Some(l) = ctx.scene.set_cursor(sender_pid, handle, code) else {
        return respond::status(sender_pid, req, E_NOENT, tx);
    };
    let c = ctx.cursor.current();
    let under = c.x >= l.x && c.y >= l.y && c.x - l.x < l.width && c.y - l.y < l.height;
    if c.visible && under && !ctx.hw_cursor() {
        ctx.damage.accumulate(reach(c.x, c.y, ctx.width, ctx.height));
    }
    respond::status(sender_pid, req, 0, tx)
}
