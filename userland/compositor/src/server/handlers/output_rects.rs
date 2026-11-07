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

//! OUTPUT_RECTS (0x0021), from the shell only: where each output sits on
//! the canvas (display/output_rects.rs), so the lock screen draws on every
//! one. Empty body; `E_PERM` from anyone else. The answer is always
//! `REPLY_LEN` bytes: the count, then four slots.

use crate::display::output_rects::{encode, rects};
use crate::protocol::{Request, E_PERM};
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &Context,
    sender: u32,
    req: &Request,
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if !super::shell_only::from_shell(sender) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let (r, n) = rects(ctx.plan.as_ref(), (ctx.width, ctx.height));
    respond::status_payload(sender, req, 0, &encode(&r[..n]), tx)
}
