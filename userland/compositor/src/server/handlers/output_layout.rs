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

//! NCMP 0x000B OUTPUT_LAYOUT (unit 10): the shell's choice for several
//! outputs. Body: mode u32, 0 mirror (every output shows the one canvas,
//! fitted) or 1 extend (one canvas across every output side by side). Only
//! desktop_shell may choose (E_PERM otherwise, looked up per request). The
//! choice is kept; with one output it waits for another, and the canvas is
//! laid out again at once otherwise. The answer's u32 says what is shown
//! now: 0 mirrored or one output, 1 extended.

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
    let Some(extend) = crate::requests::toggles::output_layout(body) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    if ctx.want_extend != extend {
        ctx.want_extend = extend;
        crate::setup::relayout(ctx);
        crate::say::say_display(ctx, "laid out on");
    }
    let shown = ctx.plan.is_some() as u32;
    respond::status_payload(sender, req, 0, &shown.to_le_bytes(), tx)
}
