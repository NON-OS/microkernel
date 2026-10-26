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

use crate::clients::wire::lookup_pid;
use crate::protocol::{read_u32, Request, E_ACCES, E_INVAL};
use crate::route::grab_rule::GRABBERS;
use crate::server::respond;
use crate::state::Context;

const E_BUSY: i32 = -16;
const REQ_LEN: usize = 8;

// A grabber is known by the pid the registry gives its name, and only the
// image's own capsules hold those names.
fn is_trusted_grabber(sender_pid: u32) -> bool {
    GRABBERS.iter().any(|name| lookup_pid(name) == Some(sender_pid))
}

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    // While the trusted band is up its holder has every key: no grab is
    // taken from it, by a named capsule or any other.
    let held = crate::route::trusted_hold::holder(ctx.trusted, nonos_libc::mk_uptime_ms());
    if held.is_some_and(|h| h != sender_pid) {
        let _ = respond::status(sender_pid, req, E_BUSY, tx);
        return;
    }
    let named = is_trusted_grabber(sender_pid);
    if !crate::route::grab_rule::grab_allowed(ctx, sender_pid, named) {
        let _ = respond::status(sender_pid, req, E_ACCES, tx);
        return;
    }
    if body.len() != REQ_LEN {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let Some(kind_mask) = read_u32(body, 0) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if kind_mask == 0 {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    if !ctx.grabs.request(sender_pid, kind_mask) {
        let _ = respond::status(sender_pid, req, E_BUSY, tx);
        return;
    }
    let _ = respond::status(sender_pid, req, 0, tx);
}
