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

//! One request to its handler; anything not served answers E_INVAL.

use super::context::Ctx;
use super::handlers::{
    bulk, bulk2, control, descriptors, hub, interrupt, poll, ports, reset, slots, transfer,
};
use super::reply::status;
use crate::env::{Clock, Mem, Regs};
use crate::wire::ops::*;
use crate::wire::{Request, E_INVAL};

pub fn dispatch<R: Regs, C: Clock, M: Mem>(
    cx: &mut Ctx<R, C, M>,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    match req.op {
        OP_HEALTHCHECK => status(tx, req, 0),
        OP_PORT_STATUS => ports::ports(cx, req, tx),
        OP_ENABLE_SLOT => ports::enable(cx, req, tx),
        OP_DISABLE_SLOT => slots::disable(cx, req, body, tx),
        OP_ADDRESS_DEVICE => slots::address(cx, req, body, tx),
        OP_GET_DEVICE_DESCRIPTOR => descriptors::device(cx, req, body, tx),
        OP_GET_CONFIG_DESCRIPTOR => descriptors::config(cx, req, body, tx),
        OP_CONTROL_TRANSFER => control::control(cx, req, body, tx),
        OP_ALLOC_TRANSFER_RING => interrupt::alloc(cx, req, body, tx),
        OP_INTERRUPT_IN => interrupt::read(cx, req, body, tx),
        OP_CONFIGURE_BULK => bulk::configure(cx, req, body, tx),
        OP_CONFIGURE_BULK2 => bulk2::configure(cx, req, body, tx),
        OP_BULK_OUT => transfer::transfer(cx, req, body, tx, false),
        OP_BULK_IN => transfer::transfer(cx, req, body, tx, true),
        OP_RESET_BULK => reset::reset(cx, req, body, tx),
        OP_BULK_IN_POLL => poll::poll(cx, req, body, tx),
        OP_ADDRESS_ROUTED => hub::address(cx, req, body, tx),
        OP_CONFIGURE_HUB => hub::configure(cx, req, body, tx),
        _ => status(tx, req, E_INVAL),
    }
}
