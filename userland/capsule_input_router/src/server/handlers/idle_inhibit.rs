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

//! IDLE_INHIBIT (0x0007): the caller holds the screen awake (body u32 1)
//! or lets it go (0), as zwp_idle_inhibit_manager_v1 asks while a surface
//! that wants it is visible. Any client may; at most `MAX_INHIBITORS` at
//! once (E_BUSY past that), and a process that exits lets go. It holds back
//! the screen blank only; it never wakes a blank screen.

use crate::protocol::{read_u32, Request, E_BUSY, E_INVAL};
use crate::server::respond;
use crate::state::Context;

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    let on = match (body.len(), read_u32(body, 0)) {
        (4, Some(v @ (0 | 1))) => v == 1,
        _ => {
            let _ = respond::status(sender_pid, req, E_INVAL, tx);
            return;
        }
    };
    let code = if ctx.idle.inhibit(sender_pid, on) { 0 } else { E_BUSY };
    let _ = respond::status(sender_pid, req, code, tx);
}
