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
use crate::state::cursor_pick::repaint;
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
    let now = ctx.cursor.current();
    for r in repaint(prev, now, ctx.hw_cursor(), (ctx.width, ctx.height)).into_iter().flatten() {
        ctx.damage.accumulate(r);
    }
    Ok(())
}
