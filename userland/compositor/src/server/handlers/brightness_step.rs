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

//! NCMP 0x0014 BRIGHTNESS_STEP: a brightness key, from desktop_shell alone
//! (E_PERM otherwise). Body: up u32, 1 or 0; any other is E_INVAL. The
//! driver's panels are asked, the focused output's (else the first listed)
//! moved to its next level that way (`display/brightness_rule.rs`), and the
//! level it is at answered as a u32 percent, unchanged at either end (0
//! beside any refusal). No
//! panel, or a driver that serves no brightness, is E_NOENT; a driver that
//! does not answer, or refuses the level, is E_IO.

use crate::display::brightness_list::{panels, set_request, MAX_PANELS};
use crate::display::brightness_rule::{next, target, OP_BRIGHTNESS};
use crate::protocol::{Request, E_INVAL, E_IO, E_NOENT, E_PERM};
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if !super::shell_only::from_shell(sender) {
        return answer(sender, req, E_PERM, 0, tx);
    }
    let Some(up) = crate::requests::toggles::brightness_step(body) else {
        return answer(sender, req, E_INVAL, 0, tx);
    };
    let Some(port) = super::driver_call::brightness_port(ctx) else {
        return answer(sender, req, E_NOENT, 0, tx);
    };
    let mut reply = [0u8; 512];
    let Some(n) = super::driver_call::call(port, &[OP_BRIGHTNESS], &mut reply) else {
        return answer(sender, req, E_IO, 0, tx);
    };
    let mut list = [None; MAX_PANELS];
    let k = panels(&reply[..n], &mut list);
    let found: alloc::vec::Vec<_> = list.iter().take(k).flatten().copied().collect();
    let out = super::driver_call::focused_output(ctx);
    let Some(p) = target(&found, out) else { return answer(sender, req, E_NOENT, 0, tx) };
    let level = match next(&p, up) {
        None => p.now,
        Some(l) if super::driver_call::status(port, &set_request(l, p.identity)) == Some(0) => l,
        Some(_) => return answer(sender, req, E_IO, 0, tx),
    };
    answer(sender, req, 0, level, tx)
}

/// Every answer carries the u32, 0 beside a refusal, so the shell reads one shape.
fn answer(
    sender: u32,
    req: &Request,
    status: i32,
    level: u8,
    tx: &mut [u8],
) -> Result<(), &'static str> {
    respond::status_payload(sender, req, status, &u32::from(level).to_le_bytes(), tx)
}
