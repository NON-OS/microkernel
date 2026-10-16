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

//! Settings' pages reach the compositor through the shell: the compositor
//! takes their ops from the shell alone, and the shell takes them from a
//! window of Settings alone (by the pid the kernel stamps on the call), so no
//! other app can turn a filter or the magnifier on, or rearrange or rescale
//! the displays. The body goes through unchanged; the compositor checks it.
//! Which ops, and what each answers, is `relay_table`.

use super::relay_table::relayed;
use crate::compositor_client::wire::{call_boot, call_payload_boot};
use crate::protocol::{Request, E_PERM};
use crate::server::handlers::instances::each_live_pid;
use crate::server::respond;
use crate::state::Context;

const SETTINGS: &[u8] = b"app.settings";
/// The compositor's answer could not be had.
const E_IO: i32 = -5;

fn from_settings(sender: u32) -> bool {
    sender != 0 && each_live_pid(SETTINGS, |pid| pid == sender)
}

/// Pass `req` on when it is a relayed op; false, with nothing sent, if not.
pub fn relay(ctx: &mut Context, sender: u32, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
    let Some((op, len)) = relayed(req.op) else { return false };
    if !from_settings(sender) {
        let _ = respond::status(sender, req, E_PERM, tx);
        return true;
    }
    let (rid, port) = (ctx.issue_request_id(), ctx.ports.compositor_port);
    if len == 0 {
        let rc = call_boot(port, op, rid, body).unwrap_or(E_IO);
        let _ = respond::status(sender, req, rc, tx);
        return true;
    }
    let mut out = [0u8; 32];
    match call_payload_boot(port, op, rid, body, &mut out[..len]) {
        Ok(0) => respond::payload(sender, req, &out[..len], tx),
        Ok(rc) => respond::status(sender, req, rc, tx),
        Err(_) => respond::status(sender, req, E_IO, tx),
    };
    true
}
