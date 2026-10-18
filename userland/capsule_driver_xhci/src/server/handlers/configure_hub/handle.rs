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

//! `OP_CONFIGURE_HUB` (`slot, ports, think_time, mtt`): tell the
//! controller an addressed slot is a hub, so it can route to the devices
//! below it and schedule split transactions for the slow ones. The hub
//! class driver parsed the hub descriptor; nothing of it is parsed here,
//! and each value is bounded before it reaches a context: 1 to 15 ports
//! (a route string field names no more), a think time of 0 to 3, MTT 0 or
//! 1. Replies with the status word.

use super::configure::configure;
use crate::contexts::{HubInfo, MAX_HUB_PORTS};
use crate::error::XhciError;
use crate::protocol::{Request, CONFIGURE_HUB_REQUEST_LEN, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;

const MAX_THINK_TIME: u8 = 3;

pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != CONFIGURE_HUB_REQUEST_LEN {
        return reply_with_status(tx, req, E_INVAL);
    }
    let (slot, ports, think_time, mtt) = (body[0], body[1], body[2], body[3]);
    if !(1..=MAX_HUB_PORTS).contains(&ports) || think_time > MAX_THINK_TIME || mtt > 1 {
        return reply_with_status(tx, req, E_INVAL);
    }
    let hub = HubInfo { ports, think_time, mtt: mtt == 1 };
    let status = match configure(ctx, slot, hub) {
        Ok(()) => 0,
        Err(XhciError::ControllerUnsupported) => E_INVAL,
        Err(_) => E_IO,
    };
    reply_with_status(tx, req, status);
}
