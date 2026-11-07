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

//! NCMP 0x001F IDLE_BLANK: the input router's word that no input came for
//! the policy's screen timeout (1), or that input came again (0). Only
//! input_router may send it (E_PERM otherwise, looked up per request); any
//! other body is E_INVAL. Blank darkens every output as a closed lid does
//! (state/lid_rule.rs); waking draws the whole canvas again, unless the lid
//! still holds it dark.

use nonos_libc::mk_service_lookup;

use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::state::Context;

const ROUTER: &[u8] = b"input_router";

fn router_pid() -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(ROUTER.as_ptr(), ROUTER.len(), &mut port, &mut pid);
    (rc >= 0 && pid != 0).then_some(pid)
}

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if sender == 0 || router_pid() != Some(sender) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let Some(blank) = crate::requests::toggles::idle_blank(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    ctx.dark_for.idle = blank;
    let (lid, whole) = ctx.lid.moved(ctx.dark_for.any());
    ctx.lid = lid;
    if whole {
        ctx.damage.mark_full(ctx.width, ctx.height);
        crate::say::say_display(
            ctx,
            if blank { "blanked after idle on" } else { "woken by input on" },
        );
    }
    respond::status(sender, req, 0, tx)
}
