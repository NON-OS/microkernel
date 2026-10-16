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

//! Settings' accessibility page reaches the compositor through the shell:
//! the compositor takes NCMP 0x000F and 0x0010 from the shell alone, and the
//! shell takes them from a window of Settings alone (by the pid the kernel
//! stamps on the call), so no other app can turn a filter or the magnifier
//! on. The body goes through unchanged; the compositor checks it.

use crate::compositor_client::wire::{call_boot, call_payload_boot};
use crate::protocol::{Request, E_PERM};
use crate::server::handlers::instances::each_live_pid;
use crate::server::respond;
use crate::state::Context;

const SETTINGS: &[u8] = b"app.settings";
const NCMP_ACCESSIBILITY: u16 = 0x000F;
const NCMP_ACCESSIBILITY_GET: u16 = 0x0010;
/// The compositor's answer could not be had.
const E_IO: i32 = -5;

fn from_settings(sender: u32) -> bool {
    sender != 0 && each_live_pid(SETTINGS, |pid| pid == sender)
}

pub fn set(ctx: &mut Context, sender: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !from_settings(sender) {
        let _ = respond::status(sender, req, E_PERM, tx);
        return;
    }
    let rid = ctx.issue_request_id();
    let rc = call_boot(ctx.ports.compositor_port, NCMP_ACCESSIBILITY, rid, body).unwrap_or(E_IO);
    let _ = respond::status(sender, req, rc, tx);
}

pub fn get(ctx: &mut Context, sender: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !from_settings(sender) {
        let _ = respond::status(sender, req, E_PERM, tx);
        return;
    }
    let rid = ctx.issue_request_id();
    let mut out = [0u8; 16];
    match call_payload_boot(ctx.ports.compositor_port, NCMP_ACCESSIBILITY_GET, rid, body, &mut out)
    {
        Ok(0) => {
            let _ = respond::payload(sender, req, &out, tx);
        }
        Ok(rc) => {
            let _ = respond::status(sender, req, rc, tx);
        }
        Err(_) => {
            let _ = respond::status(sender, req, E_IO, tx);
        }
    }
}
