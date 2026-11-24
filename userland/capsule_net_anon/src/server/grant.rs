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

//! The grant a Linux personality's guest family is served on
//! (userland/capsule_linux/design/network-grant.md). The kernel lets a
//! ForeignExec holder without Network send here and to net.socks5 alone;
//! here such a sender is served only on a grant for its live family, asked
//! for by OP_GRANT_POLICY or OP_GRANT_BOX and decided by the grant crate on
//! the kernel's facts, bound to that family's generation. It is served only
//! the ops a guest's streams take, and a stream opens only to where its grant
//! reaches: a box's own destinations, or anywhere on a policy's. A sender
//! holding Network is served as it always was.
//!
//! A box grant is decided from the personality's argv as the kernel set it.
//! The argv is read only to find the box and is never logged or kept here:
//! the row holds the box's destinations and nothing else of it.

use linux_net_grant::{decide, Ask, Facts, Grants, KernelFacts, Refusal};

use crate::protocol::{
    E_BAD_OP, E_NOT_GRANTED, E_OK, HDR_LEN, OP_CLOSE_STREAM, OP_GRANT_BOX, OP_GRANT_POLICY,
    OP_OPEN_STREAM, OP_RECV, OP_SEND,
};

use super::parse_req::parse;
use super::respond::respond;

/// The ops a granted guest family's streams take; nothing else is served it.
const GRANTED_OPS: [u16; 4] = [OP_OPEN_STREAM, OP_SEND, OP_RECV, OP_CLOSE_STREAM];

/// True when this frame was answered here: a grant asked for, or a request
/// refused. False lets it go on to be served.
pub fn gate(grants: &mut Grants, sender: u32, frame: &[u8], tx: &mut [u8]) -> bool {
    let facts = KernelFacts;
    let Ok((req, body)) = parse(frame) else {
        if facts.holds_network(sender) {
            return false;
        }
        respond(sender, E_BAD_OP, E_NOT_GRANTED, 0, 0, tx);
        return true;
    };
    let ask = match req.op {
        OP_GRANT_POLICY => Some(Ask::Policy(body)),
        OP_GRANT_BOX => Some(Ask::Box),
        _ => None,
    };
    if let Some(ask) = ask {
        let made = decide(sender, ask, &facts).and_then(|(g, n)| grants.insert(sender, n, g));
        let (errno, why) = match made {
            Ok(()) => (E_OK, linux_net_grant::wire::GRANTED),
            Err(r) => (E_NOT_GRANTED, Refusal::code(r)),
        };
        tx[HDR_LEN] = why;
        respond(sender, req.op, errno, req.request_id, 1, tx);
        return true;
    }
    if facts.holds_network(sender) {
        return false;
    }
    let served = match req.op {
        OP_OPEN_STREAM => {
            opens_to(body).is_some_and(|(host, port)| grants.may_reach(sender, host, port, &facts))
        }
        op => GRANTED_OPS.contains(&op) && grants.admits(sender, &facts),
    };
    if !served {
        respond(sender, req.op, E_NOT_GRANTED, req.request_id, 0, tx);
    }
    !served
}

/// Where an OP_OPEN_STREAM asks to go: its port, two bytes, then its host
/// (handlers/open.rs).
fn opens_to(body: &[u8]) -> Option<(&[u8], u16)> {
    let (port, host) = (body.get(..2)?, body.get(2..)?);
    (!host.is_empty()).then(|| (host, u16::from_le_bytes([port[0], port[1]])))
}
