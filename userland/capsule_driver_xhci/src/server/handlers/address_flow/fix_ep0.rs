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

//! A full-speed device is addressed with an EP0 of 8 bytes and may use 16,
//! 32 or 64. Its first 8 device descriptor bytes, one packet at any size,
//! give bMaxPacketSize0, and Evaluate Context hands that to the controller
//! (xHCI 1.2 section 4.6.5 note, Linux `hub_port_init`). Left at 8, the
//! first longer read from a 64-byte device (its configuration descriptor)
//! ends in a babble error on silicon.

use crate::contexts::{ep0_max_packet, ep0_needs_descriptor, write_evaluate_ep0_input};
use crate::controller::{issue_evaluate_context, read_device_descriptor, DEVICE_DESCRIPTOR_PREFIX};
use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;
use crate::server::handlers::recover::{recover_after, DCI_EP0};

/// bMaxPacketSize0's offset in the device descriptor.
const B_MAX_PACKET_SIZE0: usize = 7;

/// Settle EP0's size for the addressed `slot`; returns the size in use.
pub(super) fn fix_ep0(ctx: &mut Context, slot: u8) -> XhciResult<u16> {
    let d = &mut ctx.driver;
    let res =
        d.slots.resources_mut(slot, d.layout.max_slots).ok_or(XhciError::ControllerUnsupported)?;
    if !ep0_needs_descriptor(res.speed, res.usb3) {
        return Ok(res.max_packet);
    }
    let buf = d.dma_pool.alloc(DEVICE_DESCRIPTOR_PREFIX as u64)?;
    buf.zero();
    let (db, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    let read = read_device_descriptor(
        db,
        intr,
        &mut d.event_ring,
        slot,
        &mut res.ep0,
        &buf,
        DEVICE_DESCRIPTOR_PREFIX,
    );
    if let Err(e) = read {
        recover_after(ctx, slot, DCI_EP0, e);
        return Err(e);
    }
    /*
     * SAFETY: `buf` is a live DMA grant of at least 8 bytes and the
     * transfer into it has completed.
     */
    let b = unsafe { core::ptr::read_volatile(buf.as_mut_ptr::<u8>().add(B_MAX_PACKET_SIZE0)) };
    let Some(mps) = ep0_max_packet(res.speed, res.usb3, b) else { return Ok(res.max_packet) };
    if mps == res.max_packet {
        return Ok(mps);
    }
    let csz = d.layout.context_size;
    write_evaluate_ep0_input(&res.input_context, &res.output_context, csz, mps);
    let input = res.input_context.phys();
    issue_evaluate_context(db, intr, &mut d.command_ring, &mut d.event_ring, input, slot)?;
    res.max_packet = mps;
    Ok(mps)
}
