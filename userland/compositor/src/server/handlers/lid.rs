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

//! NCMP 0x0013 LID: the shell's word that the lid closed or opened. Body:
//! closed u32, 1 closed or 0 open. Only desktop_shell may send it (E_PERM
//! otherwise, looked up per request); any other body is E_INVAL. Closed,
//! every output is composed black once and then nothing is presented; open,
//! the whole canvas is damaged and drawn again (state/lid_rule.rs). Saying
//! what the lid already is changes nothing.

use nonos_libc::mk_service_lookup;

use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::state::Context;

const SHELL: &[u8] = b"desktop_shell";

fn shell_pid() -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(SHELL.as_ptr(), SHELL.len(), &mut port, &mut pid);
    (rc >= 0 && pid != 0).then_some(pid)
}

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if sender == 0 || shell_pid() != Some(sender) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let Some(closed) = crate::requests::toggles::lid(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    let (lid, whole) = ctx.lid.moved(closed);
    ctx.lid = lid;
    if whole {
        ctx.damage.mark_full(ctx.width, ctx.height);
        crate::say::say_display(
            ctx,
            if closed { "darkened for the lid on" } else { "woken by the lid on" },
        );
    }
    respond::status(sender, req, 0, tx)
}
