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
//! Bringing a bulk endpoint back for the class driver's recovery: Reset
//! Endpoint, or Stop Endpoint when it had not halted, then the dequeue
//! pointer moved past the TRBs that will never complete. The class driver
//! clears the device's halt feature over the control pipe afterwards, which
//! sets the device's data toggle (USB 2) or sequence number (USB 3) back to
//! zero. Reset Endpoint does the same on the host side; when the endpoint
//! was only stopped, the endpoint is dropped and added again so the host
//! starts from zero too, or the next packet would go out with the toggle the
//! device no longer expects.
use crate::controller::recover_endpoint::{recover_endpoint, Recovered};
use crate::controller::reset_toggle::{reset_toggle, ToggleReset};
use crate::error::{XhciError, XhciResult};
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::slots::SlotResources;

pub fn reset_bulk_endpoint(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    res: &SlotResources,
    context_size: u8,
    dir_in: bool,
) -> XhciResult<()> {
    let pipes = res.bulk.as_ref().ok_or(XhciError::ControllerUnsupported)?;
    let (ring, dci) =
        if dir_in { (&pipes.in_ring, pipes.dci_in) } else { (&pipes.out_ring, pipes.dci_out) };
    let slot = res.slot_id;
    if recover_endpoint(doorbell_base, intr_base, cmd_ring, evt_ring, ring, slot, dci)?
        == Recovered::Reset
    {
        return Ok(());
    }
    let ep = ToggleReset {
        input: &res.input_context,
        output: &res.output_context,
        context_size,
        ring,
        slot,
        dci,
    };
    reset_toggle(doorbell_base, intr_base, cmd_ring, evt_ring, ep)
}
