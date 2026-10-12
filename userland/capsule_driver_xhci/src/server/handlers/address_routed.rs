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
//! OP_ADDRESS_ROUTED: `slot, root_port, speed, route_le32, tt_slot,
//! tt_port`, answered as OP_ADDRESS_DEVICE is. Refused unless a hub holds
//! the root port, the route is well formed, and a TT, when named, is a
//! hub's port and serves a low or full speed device.

use super::address_flow::{address_routed, slot_ready, Below};
use crate::contexts::routed_fits;
use crate::protocol::{Request, E_INVAL};
use crate::server::context::Context;
use crate::server::error::reply_with_status;

const REQUEST_LEN: usize = 9;

pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let Ok(&[slot, port, speed, r0, r1, r2, r3, tt_slot, tt_port]) =
        <&[u8; REQUEST_LEN]>::try_from(body)
    else {
        return reply_with_status(tx, req, E_INVAL);
    };
    let route = u32::from_le_bytes([r0, r1, r2, r3]);
    let tt = (tt_slot != 0).then_some((tt_slot, tt_port));
    let slots = &ctx.driver.slots;
    let tt_is_hub = tt.is_some_and(|(s, _)| slots.is_hub(s));
    let ok = slot_ready(ctx, slot, port)
        && slots.hub_on(port)
        && routed_fits(speed, route, tt, tt_is_hub);
    if !ok {
        return reply_with_status(tx, req, E_INVAL);
    }
    address_routed(ctx, req, tx, (slot, port, speed), Below { route, tt });
}
