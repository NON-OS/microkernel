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

//! After a transfer failed or never finished, the endpoint it ran on is
//! brought back (`recover_endpoint`) before the reply goes out, so the class
//! driver's next request on it is served. Before this a STALLed control
//! request, which many HID devices answer to SET_IDLE, left EP0 halted and
//! every later request on the device timed out.

use crate::controller::recover_endpoint;
use crate::error::XhciError;
use crate::server::context::Context;

/// The DCI of the default control endpoint.
pub const DCI_EP0: u8 = 1;

/// Recover endpoint `dci` of `slot` if `err` left it halted or running on
/// TRBs that will never complete. Other errors left nothing behind.
pub fn recover_after(ctx: &mut Context, slot: u8, dci: u8, err: XhciError) {
    if !matches!(err, XhciError::TransferCompletionFailed(_) | XhciError::TransferCompletionTimeout)
    {
        return;
    }
    let d = &mut ctx.driver;
    let Some(res) = d.slots.resources_mut(slot, d.layout.max_slots) else { return };
    let ring = if dci == DCI_EP0 {
        &res.ep0
    } else if let Some(ep) = res.interrupt.iter_mut().find(|ep| ep.dci == dci) {
        ep.armed = None;
        &ep.ring
    } else if let Some(pipes) = res.bulk.as_ref() {
        if dci == pipes.dci_in {
            &pipes.in_ring
        } else if dci == pipes.dci_out {
            &pipes.out_ring
        } else {
            return;
        }
    } else {
        return;
    };
    let _ = recover_endpoint(
        d.layout.doorbell_base,
        d.layout.primary_intr_base,
        &mut d.command_ring,
        &mut d.event_ring,
        ring,
        slot,
        dci,
    );
}
