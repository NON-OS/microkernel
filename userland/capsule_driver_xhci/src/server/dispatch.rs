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
use crate::protocol::{
    Request, E_INVAL, E_NODEV, OP_ADDRESS_DEVICE, OP_ADDRESS_ROUTED, OP_ALLOC_TRANSFER_RING,
    OP_BULK_IN, OP_BULK_OUT, OP_CONFIGURE_BULK, OP_CONFIGURE_HUB, OP_CONTROLLER_STATUS,
    OP_CONTROL_TRANSFER, OP_DISABLE_SLOT, OP_ENABLE_SLOT, OP_GET_CONFIG_DESCRIPTOR,
    OP_GET_DEVICE_DESCRIPTOR, OP_HEALTHCHECK, OP_INTERRUPT_IN, OP_PORT_STATUS, OP_RESET_BULK,
};
use crate::server::{context::Context, error::reply_with_status, handlers, reply::sender};

/// Serve one request. Whether the controller failed is looked at before and
/// after each. A failed one is brought back before a request that needs
/// it, and while it cannot be every such request is refused with E_NODEV;
/// status requests are answered either way.
pub fn dispatch(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let status = matches!(req.op, OP_HEALTHCHECK | OP_CONTROLLER_STATUS | OP_PORT_STATUS);
    if ctx.driver.failed() {
        ctx.driver.dead = true;
    }
    if ctx.driver.dead && !status && ctx.driver.revive().is_err() {
        return reply_with_status(tx, req, E_NODEV);
    }
    route(ctx, req, body, tx);
    if ctx.driver.failed() {
        ctx.driver.dead = true;
    }
}

fn route(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(req, tx),
        OP_CONTROLLER_STATUS if body.is_empty() => {
            handlers::controller_status::handle(ctx, req, tx)
        }
        OP_PORT_STATUS if body.is_empty() => handlers::port_status::handle(ctx, req, tx),
        OP_ENABLE_SLOT if body.is_empty() => handlers::enable_slot::handle(ctx, req, tx),
        _ if !sender_holds(ctx, body) => reply_with_status(tx, req, E_INVAL),
        OP_DISABLE_SLOT => handlers::disable_slot::handle(ctx, req, body, tx),
        OP_ADDRESS_DEVICE => handlers::address_device::handle(ctx, req, body, tx),
        OP_ADDRESS_ROUTED => handlers::address_routed::handle(ctx, req, body, tx),
        OP_CONFIGURE_HUB => handlers::configure_hub::handle(ctx, req, body, tx),
        OP_GET_DEVICE_DESCRIPTOR => handlers::device_descriptor::handle(ctx, req, body, tx),
        OP_GET_CONFIG_DESCRIPTOR => handlers::config_descriptor::handle(ctx, req, body, tx),
        OP_CONTROL_TRANSFER => handlers::control_transfer::handle(ctx, req, body, tx),
        OP_ALLOC_TRANSFER_RING => handlers::alloc_transfer_ring::handle(ctx, req, body, tx),
        OP_INTERRUPT_IN => handlers::interrupt_in::handle(ctx, req, body, tx),
        OP_CONFIGURE_BULK => handlers::bulk::configure(ctx, req, body, tx),
        OP_BULK_OUT => handlers::bulk::transfer_out(ctx, req, body, tx),
        OP_BULK_IN => handlers::bulk::transfer_in(ctx, req, body, tx),
        OP_RESET_BULK => handlers::bulk::reset(ctx, req, body, tx),
        _ => reply_with_status(tx, req, E_INVAL),
    }
}

/// Every other request names a slot in its first byte, one the sender must
/// have enabled (F4): a revival hands the same ids out again, and a stale
/// one reached another client's new device.
fn sender_holds(ctx: &Context, body: &[u8]) -> bool {
    let (slots, max) = (&ctx.driver.slots, ctx.driver.layout.max_slots);
    body.first().is_some_and(|&slot| slots.held_by(slot, max, sender()))
}
