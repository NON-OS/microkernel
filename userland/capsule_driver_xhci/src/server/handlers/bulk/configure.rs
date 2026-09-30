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
//! `OP_CONFIGURE_BULK`: add a bulk IN and a bulk OUT endpoint to an
//! addressed slot. Request `slot, ep_in, ep_out, 0, mps_in_le16,
//! mps_out_le16`; reply `dci_in, dci_out, 0, 0`.
use super::reply::send_prefix;
use crate::controller::{issue_configure_endpoint, write_bulk_input, BulkEndpoint, BulkPipes};
use crate::error::{XhciError, XhciResult};
use crate::protocol::{Request, CONFIGURE_BULK_REQUEST_LEN, E_INVAL, E_IO};
use crate::server::context::Context;
use crate::server::error::reply_with_status;
use crate::slots::dci_from_ep_address;
pub fn configure(ctx: &mut Context, req: &Request, body: &[u8], tx: &mut [u8]) {
    let well_formed = body.len() == CONFIGURE_BULK_REQUEST_LEN
        && body[1] & 0x80 != 0
        && body[2] & 0x80 == 0
        && body[1] & 0x0F != 0
        && body[2] & 0x0F != 0;
    if !well_formed {
        return reply_with_status(tx, req, E_INVAL);
    }
    let (dci_in, dci_out) = (dci_from_ep_address(body[1]), dci_from_ep_address(body[2]));
    let mps = |at: usize| u16::from_le_bytes([body[at], body[at + 1]]);
    match run(ctx, body[0], (dci_in, mps(4)), (dci_out, mps(6))) {
        Ok(()) => send_prefix(tx, req, [dci_in, dci_out, 0, 0]),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
fn run(ctx: &mut Context, slot: u8, bulk_in: (u8, u16), bulk_out: (u8, u16)) -> XhciResult<()> {
    let pipes = BulkPipes::new(&ctx.driver.dma_pool, bulk_in.0, bulk_out.0)?;
    let context_size = ctx.driver.layout.context_size;
    let max_slots = ctx.driver.layout.max_slots;
    let res =
        ctx.driver.slots.resources_mut(slot, max_slots).ok_or(XhciError::ControllerUnsupported)?;
    if res.bulk.is_some() || res.int_ring.is_some() {
        return Err(XhciError::ControllerUnsupported);
    }
    let ep_in =
        BulkEndpoint { dci: bulk_in.0, ring_phys: pipes.in_ring.phys(), max_packet: bulk_in.1 };
    let ep_out =
        BulkEndpoint { dci: bulk_out.0, ring_phys: pipes.out_ring.phys(), max_packet: bulk_out.1 };
    write_bulk_input(&res.input_context, context_size, (res.speed, res.port_id), ep_in, ep_out);
    let input_phys = res.input_context.phys();
    issue_configure_endpoint(
        ctx.driver.layout.doorbell_base,
        ctx.driver.layout.primary_intr_base,
        &mut ctx.driver.command_ring,
        &mut ctx.driver.event_ring,
        input_phys,
        slot,
    )?;
    if let Some(res) = ctx.driver.slots.resources_mut(slot, max_slots) {
        res.bulk = Some(pipes);
    }
    Ok(())
}
