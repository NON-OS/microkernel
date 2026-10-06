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
use super::address_flow::{address_after_reset, port_speed, slot_ready};
use crate::controller::reset_port;
use crate::protocol::{Request, ADDRESS_DEVICE_REQUEST_LEN, E_BUSY, E_INVAL, E_IO, E_NODEV};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::PORT_FREE;
pub fn handle(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != ADDRESS_DEVICE_REQUEST_LEN {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    let slot_id = body[0];
    let port_id = body[1];
    if !slot_ready(ctx, slot_id, port_id) {
        reply_with_status(tx, req, E_INVAL);
        return;
    }
    if ctx.driver.slots.port_state(port_id) != PORT_FREE {
        reply_with_status(tx, req, E_BUSY);
        return;
    }
    let usb3 = ctx.driver.layout.ports.is_usb3(port_id);
    let portsc = match reset_port(ctx.driver.layout.op_base, port_id, usb3) {
        Ok(v) => v,
        Err(crate::error::XhciError::NoDeviceOnPort) => {
            reply_with_status(tx, req, E_NODEV);
            return;
        }
        Err(_) => {
            reply_with_status(tx, req, E_IO);
            return;
        }
    };
    address_after_reset(ctx, req, tx, slot_id, port_id, port_speed(portsc));
}
