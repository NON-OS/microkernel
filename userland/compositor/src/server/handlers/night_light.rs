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

//! NCMP 0x000E NIGHT_LIGHT: the shell's colour temperature for the screen,
//! kelvin u32 (0 or 6500 and above: true colour; 1000 the warmest). The
//! shell keeps the schedule (sunset, a fade) and sends each step; the
//! compositor looks every composed pixel up through the ramps and repaints
//! the whole canvas. Only desktop_shell may send it (E_PERM otherwise).

use nonos_libc::mk_service_lookup;

use crate::display::night_rule::{active, ramps, WARMEST_K};
use crate::protocol::{Request, E_INVAL, E_PERM, NIGHT_LIGHT_REQ_LEN};
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
    let k = super::u32_at(body, 0).filter(|_| body.len() == NIGHT_LIGHT_REQ_LEN);
    let Some(k) = k.filter(|&k| k == 0 || k >= WARMEST_K) else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    ctx.looks.night = active(k).then(|| alloc::boxed::Box::new(ramps(k)));
    ctx.looks.rebuild();
    ctx.damage.mark_full(ctx.width, ctx.height);
    respond::status(sender, req, 0, tx)
}
