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
//! addressed slot. Request `slot, ep_in, ep_out, bursts, mps_in_le16,
//! mps_out_le16`; reply `dci_in, dci_out, 0, 0`. `bursts` holds the
//! SuperSpeed Endpoint Companion's bMaxBurst of the IN endpoint in bits 3:0
//! and of the OUT endpoint in bits 7:4; it was a reserved zero, which still
//! means one packet per burst, and it is ignored below SuperSpeed.
use super::reply::send_prefix;
use crate::contexts::{is_superspeed, SPEED_HIGH};
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
    let bursts = Bursts { bulk_in: body[3] & 0x0F, bulk_out: body[3] >> 4 };
    match run(ctx, body[0], (dci_in, mps(4)), (dci_out, mps(6)), bursts) {
        Ok(()) => send_prefix(tx, req, [dci_in, dci_out, 0, 0]),
        Err(XhciError::ControllerUnsupported) => reply_with_status(tx, req, E_INVAL),
        Err(_) => reply_with_status(tx, req, E_IO),
    }
}
#[derive(Clone, Copy)]
struct Bursts {
    bulk_in: u8,
    bulk_out: u8,
}

fn run(
    ctx: &mut Context,
    slot: u8,
    bulk_in: (u8, u16),
    bulk_out: (u8, u16),
    bursts: Bursts,
) -> XhciResult<()> {
    let pipes = BulkPipes::new(&ctx.driver.dma_pool, bulk_in.0, bulk_out.0)?;
    let context_size = ctx.driver.layout.context_size;
    let max_slots = ctx.driver.layout.max_slots;
    let res =
        ctx.driver.slots.resources_mut(slot, max_slots).ok_or(XhciError::ControllerUnsupported)?;
    if res.bulk.is_some() || !res.interrupt.is_empty() {
        return Err(XhciError::ControllerUnsupported);
    }
    let (speed, usb3) = (res.speed, res.usb3);
    let burst = |b: u8| if is_superspeed(speed, usb3) { b } else { 0 };
    let ep_in = BulkEndpoint {
        dci: bulk_in.0,
        ring_phys: pipes.in_ring.phys(),
        max_packet: bulk_max_packet(speed, usb3, bulk_in.1),
        max_burst: burst(bursts.bulk_in),
    };
    let ep_out = BulkEndpoint {
        dci: bulk_out.0,
        ring_phys: pipes.out_ring.phys(),
        max_packet: bulk_max_packet(speed, usb3, bulk_out.1),
        max_burst: burst(bursts.bulk_out),
    };
    write_bulk_input(&res.input_context, &res.output_context, context_size, ep_in, ep_out);
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
/// The bulk max packet size the controller is given. USB 2.0 fixes it at
/// 512 for high speed and USB 3 at 1024 for SuperSpeed, and some USB sticks
/// report another value; Linux's `xhci_endpoint_init` overrides it the same
/// way ("Some devices get this wrong"). Full speed keeps what the descriptor
/// says, bits 10:0.
fn bulk_max_packet(speed: u8, usb3: bool, descriptor: u16) -> u16 {
    if is_superspeed(speed, usb3) {
        1024
    } else if speed == SPEED_HIGH {
        512
    } else {
        descriptor & 0x07FF
    }
}
