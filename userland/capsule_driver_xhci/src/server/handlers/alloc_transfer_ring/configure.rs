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

use crate::contexts::{interrupt_interval, write_configure_endpoint_input, EndpointConfig};
use crate::controller::issue_configure_endpoint;
use crate::error::{XhciError, XhciResult};
use crate::rings::transfer::TransferRing;
use crate::server::context::Context;
use crate::slots::{interrupt_limit, InterruptEndpoint};

/// wMaxPacketSize bits 10:0; bits 12:11 are high-bandwidth transactions.
const MAX_PACKET_MASK: u16 = 0x07FF;

/// Add the interrupt endpoint at `dci` to `slot`, `b_interval` converted by
/// the device's speed; one already there keeps its ring, its buffer wiped.
pub(super) fn do_configure(
    ctx: &mut Context,
    slot: u8,
    dci: u8,
    max_packet: u16,
    b_interval: u8,
) -> XhciResult<()> {
    let max_packet = max_packet & MAX_PACKET_MASK;
    if dci < 2 || max_packet == 0 {
        return Err(XhciError::ControllerUnsupported);
    }
    let d = &mut ctx.driver;
    let res =
        d.slots.resources_mut(slot, d.layout.max_slots).ok_or(XhciError::ControllerUnsupported)?;
    // A sharer's driver restarted: no former holder's report is left.
    if let Some(ep) = res.interrupt_mut(dci) {
        ep.buf.zero();
        return Ok(());
    }
    if !res.takes_interrupt() {
        return Err(XhciError::ControllerUnsupported);
    }
    let ring = TransferRing::new(&d.dma_pool)?;
    let max = interrupt_limit(max_packet);
    let buf = d.dma_pool.alloc(max as u64)?;
    let cfg = EndpointConfig {
        context_size: d.layout.context_size,
        dci,
        ring_phys: ring.phys(),
        max_packet,
        interval: interrupt_interval(res.speed, res.usb3, b_interval),
    };
    write_configure_endpoint_input(&res.input_context, &res.output_context, cfg);
    let input = res.input_context.phys();
    issue_configure_endpoint(
        d.layout.doorbell_base,
        d.layout.primary_intr_base,
        &mut d.command_ring,
        &mut d.event_ring,
        input,
        slot,
    )?;
    if let Some(res) = d.slots.resources_mut(slot, d.layout.max_slots) {
        res.interrupt.push(InterruptEndpoint { dci, ring, buf, max, armed: None });
    }
    Ok(())
}
