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

//! `OP_ADDRESS_ROUTED` (`slot, root_port, speed, route_le32, tt_hub_slot,
//! tt_port`): Address Device for a device below a hub (xHCI 1.2 section
//! 4.3.3). The root port is not reset: the hub reset its own port, and
//! resetting the root port would reset the hub. Every field is the
//! client's, and is held against what this driver set up (`place`). The
//! reply is OP_ADDRESS_DEVICE's.

use super::place::place_below_hub;
use crate::protocol::{Request, ADDRESS_ROUTED_REQUEST_LEN, E_BUSY, E_INVAL};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::address_flow::{address_at, slot_ready};

pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != ADDRESS_ROUTED_REQUEST_LEN {
        return reply_with_status(tx, req, E_INVAL);
    }
    let (slot, root_port, speed) = (body[0], body[1], body[2]);
    let route = u32::from_le_bytes([body[3], body[4], body[5], body[6]]);
    if !slot_ready(ctx, slot, root_port) {
        return reply_with_status(tx, req, E_INVAL);
    }
    let Some(place) = place_below_hub(ctx, root_port, speed, route, (body[7], body[8])) else {
        return reply_with_status(tx, req, E_INVAL);
    };
    // One device per hub port, as one per root port: the route names it.
    if ctx.driver.slots.at_route(root_port, route).is_some() {
        return reply_with_status(tx, req, E_BUSY);
    }
    address_at(ctx, req, tx, slot, place);
}
