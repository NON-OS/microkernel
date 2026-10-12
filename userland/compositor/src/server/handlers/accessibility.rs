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

//! NCMP 0x000F ACCESSIBILITY: one output's colour filter, high contrast
//! and magnifier, as Settings chose them (`display/access_rule.rs` has the
//! body). Only desktop_shell may send it (E_PERM otherwise); an output past
//! the four the wire serves, or a value outside its steps, is E_INVAL. The
//! output's look is built again and the canvas repainted; turning the
//! magnifier on or off lays the canvas out again, since the lens reads a
//! canvas of its own.

use crate::display::access_rule::parse;
use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::server::trusted_pids::trusted_pids;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if sender == 0 || !trusted_pids().contains(&Some(sender)) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let Some((out, a)) = parse(body, ctx.looks.access.len()) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    let before = ctx.looks.access[out];
    if out == 0 && a.magnified() && ctx.hw_cursor() {
        crate::frame_pacer::hw_cursor_back::hide(ctx);
    }
    ctx.looks.access[out] = a;
    ctx.looks.rebuild();
    if before.magnified() != a.magnified() {
        crate::setup::relayout(ctx);
    }
    ctx.damage.mark_full(ctx.width, ctx.height);
    respond::status(sender, req, 0, tx)
}
