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

//! `OP_CONFIGURE_ISOCH`: add one isochronous OUT endpoint to an addressed
//! slot. Request `slot, ep_address, bInterval, max_burst, mps_le16, mult, 0`
//! (`mps` is wMaxPacketSize whole; `max_burst` and `mult` the SuperSpeed
//! companion's, ignored below it). Reply `dci, 0, 0, 0`.

use crate::contexts::is_superspeed;
use crate::controller::{
    isoch_endpoint, issue_configure_endpoint, write_isoch_input, IsochEndpoint, IsochPipe,
};
use crate::error::{XhciError, XhciResult};
use crate::protocol::{Request, CONFIGURE_ISOCH_REQUEST_LEN, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::server::handlers::bulk::reply::send_prefix;
use crate::slots::dci_from_ep_address;

pub fn configure(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let ok = body.len() == CONFIGURE_ISOCH_REQUEST_LEN
        && body[1] & 0x80 == 0
        && body[1] & 0x0F != 0
        && (1..=16).contains(&body[2]);
    if !ok {
        return reply_with_status(tx, req, E_INVAL);
    }
    let (dci, mps) = (dci_from_ep_address(body[1]), u16::from_le_bytes([body[4], body[5]]));
    match run(ctx, body[0], dci, body[2], (body[3], body[6]), mps) {
        Ok(()) => send_prefix(tx, req, [dci, 0, 0, 0]),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}

fn run(
    ctx: &mut Context,
    slot: u8,
    dci: u8,
    b_interval: u8,
    ss: (u8, u8),
    mps: u16,
) -> XhciResult<()> {
    let d = &mut ctx.driver;
    let (context_size, max_slots) = (d.layout.context_size, d.layout.max_slots);
    let res = d.slots.resources_mut(slot, max_slots).ok_or(XhciError::ControllerUnsupported)?;
    if res.isoch.is_some() || mps & 0x07FF == 0 {
        return Err(XhciError::ControllerUnsupported);
    }
    // Allocated only for a request that will be served: the pool lends once.
    let (ring, buf) = IsochPipe::memory(&d.dma_pool)?;
    let ep = isoch_endpoint(res.speed, res.usb3, dci, b_interval, ss, mps);
    let ep = IsochEndpoint { ring_phys: ring.phys(), ..ep };
    write_isoch_input(&res.input_context, &res.output_context, context_size, ep);
    let (input_phys, superspeed) = (res.input_context.phys(), is_superspeed(res.speed, res.usb3));
    let (db, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    issue_configure_endpoint(db, intr, &mut d.command_ring, &mut d.event_ring, input_phys, slot)?;
    if let Some(res) = d.slots.resources_mut(slot, max_slots) {
        res.isoch = Some(IsochPipe::with(ring, buf, ep, superspeed));
    }
    Ok(())
}
