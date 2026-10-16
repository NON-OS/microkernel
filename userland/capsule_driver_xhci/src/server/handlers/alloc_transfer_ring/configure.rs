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

use crate::contexts::{
    interrupt_interval, interrupt_payload, write_configure_endpoint_input, EndpointConfig,
};
use crate::controller::issue_configure_endpoint;
use crate::error::{XhciError, XhciResult};
use crate::rings::transfer::TransferRing;
use crate::server::context::Context;
use crate::server::handlers::recover::keep_if_failed;
use crate::slots::{InterruptEndpoint, MAX_INTERRUPT_ENDPOINTS};

/// Add the interrupt-IN endpoint at `dci` to `slot`, from its descriptor's
/// `w_max_packet` and `b_interval` (converted here by the device's speed)
/// and, on SuperSpeed, its companion's `ss_burst`. Its report buffer holds
/// the endpoint's whole payload per interval, so a report as long as the
/// endpoint allows never babbles. An endpoint already configured is left
/// as it is; an OUT DCI, or a payload past the specification, is refused.
pub(super) fn do_configure(
    ctx: &mut Context,
    slot: u8,
    dci: u8,
    w_max_packet: u16,
    b_interval: u8,
    ss_burst: u8,
) -> XhciResult<()> {
    let d = &mut ctx.driver;
    let res =
        d.slots.resources_mut(slot, d.layout.max_slots).ok_or(XhciError::ControllerUnsupported)?;
    let payload = interrupt_payload(res.speed, res.usb3, w_max_packet, ss_burst)
        .filter(|_| dci >= 3 && dci % 2 == 1)
        .ok_or(XhciError::ControllerUnsupported)?;
    if res.interrupt_mut(dci).is_some() {
        return Ok(());
    }
    if res.interrupt.len() >= MAX_INTERRUPT_ENDPOINTS || res.bulk.is_some() {
        return Err(XhciError::ControllerUnsupported);
    }
    let ring = TransferRing::new(&d.dma_pool)?;
    let buf = d.dma_pool.alloc(payload.esit as u64)?;
    let cfg = EndpointConfig {
        context_size: d.layout.context_size,
        dci,
        ring_phys: ring.phys(),
        payload,
        interval: interrupt_interval(res.speed, res.usb3, b_interval),
    };
    write_configure_endpoint_input(&res.input_context, &res.output_context, cfg);
    let input = res.input_context.phys();
    let (db, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    let (cmd, evt) = (&mut d.command_ring, &mut d.event_ring);
    if let Err(e) = issue_configure_endpoint(db, intr, cmd, evt, input, slot) {
        return Err(keep_if_failed(ctx, slot, e, ring.into_regions().chain([buf])));
    }
    if let Some(res) = d.slots.resources_mut(slot, d.layout.max_slots) {
        let len = payload.esit;
        res.interrupt.push(InterruptEndpoint { dci, ring, buf, len, armed: None });
    }
    Ok(())
}
