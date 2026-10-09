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
use crate::protocol::HID_REPORT_MAX;
use crate::rings::transfer::TransferRing;
use crate::server::context::Context;
use crate::slots::{InterruptEndpoint, MAX_INTERRUPT_ENDPOINTS};

/// wMaxPacketSize bits 10:0; bits 12:11 are high-bandwidth transactions.
const MAX_PACKET_MASK: u16 = 0x07FF;

/// Add the interrupt-IN endpoint at `dci` to `slot`. `b_interval` is the
/// endpoint descriptor's, converted here by the device's speed. An endpoint
/// already configured is left as it is.
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
    if res.interrupt_mut(dci).is_some() {
        return Ok(());
    }
    if res.interrupt.len() >= MAX_INTERRUPT_ENDPOINTS || res.bulk.is_some() {
        return Err(XhciError::ControllerUnsupported);
    }
    let ring = TransferRing::new(&d.dma_pool)?;
    let buf = d.dma_pool.alloc(HID_REPORT_MAX as u64)?;
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
        res.interrupt.push(InterruptEndpoint { dci, ring, buf, armed: None });
    }
    Ok(())
}
