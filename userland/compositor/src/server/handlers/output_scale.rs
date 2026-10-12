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

//! NCMP 0x000D OUTPUT_SCALE: the shell's scale for one output. Body:
//! output u32 (0 the first), quarters u32 (0 back to the panel's own rule,
//! else one of 4, 5, 6, 8, 12, 16: 1, 1.25, 1.5, 2, 3 and 4 times). Only
//! desktop_shell may choose (E_PERM otherwise); an output past the four the
//! wire serves or another step is E_INVAL. The canvas is laid out again at
//! once, with the pointer and damage mapped through the new scale, and
//! clients learn the new canvas from DISPLAY_INFO. The answer's u32 is the
//! output 0 scale now in quarters.

use nonos_libc::mk_service_lookup;

use crate::display::scale_rule::STEPS;
use crate::protocol::{Request, E_INVAL, E_PERM, OUTPUT_SCALE_REQ_LEN};
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
    let (Some(out), Some(q)) = (super::u32_at(body, 0), super::u32_at(body, 4)) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    let ok = body.len() == OUTPUT_SCALE_REQ_LEN && (q == 0 || STEPS.contains(&q));
    let Some(slot) = ctx.output_scale.get_mut(out as usize).filter(|_| ok) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    let want = (q != 0).then_some(q);
    if *slot != want {
        *slot = want;
        crate::setup::relayout(ctx);
        crate::say::say_display(ctx, "scaled on");
    }
    respond::status_payload(sender, req, 0, &ctx.scale_q.to_le_bytes(), tx)
}
