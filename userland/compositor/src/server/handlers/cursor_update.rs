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

use crate::protocol::Request;
use crate::state::cursor_pick::reach;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    _sender_pid: u32,
    _req: &Request,
    body: &[u8],
    _tx: &mut [u8],
) -> Result<(), &'static str> {
    let Some(c) = crate::requests::pointer::cursor_update(body) else {
        return Ok(());
    };
    if !c.on(ctx.width, ctx.height) {
        return Ok(());
    }
    let (x, y, visible) = (c.x, c.y, c.visible);
    let prev = ctx.cursor.update(x, y, visible);
    if ctx.hw_cursor() {
        // The next frame moves the device's plane; no pixel changed.
        return Ok(());
    }
    // Each shape is drawn from its hot spot, so the box reaches back from it.
    if prev.visible {
        ctx.damage.accumulate(reach(prev.x, prev.y, ctx.width, ctx.height));
    }
    if visible {
        ctx.damage.accumulate(reach(x, y, ctx.width, ctx.height));
    }
    Ok(())
}
