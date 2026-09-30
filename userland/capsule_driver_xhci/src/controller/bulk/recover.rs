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
//! Bringing a halted bulk endpoint back after a STALL: Reset Endpoint, then
//! the dequeue pointer moved past the TRB that stalled. The class driver
//! clears the device's halt feature over the control pipe afterwards.
use super::commands::{reset_endpoint_command, set_dequeue_command};
use super::pipes::BulkPipes;
use crate::controller::ring_doorbell::ring_doorbell;
use crate::controller::wait_command_completion::wait_command_completion;
use crate::error::XhciResult;
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::trb::Trb;
pub fn reset_bulk_endpoint(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    pipes: &BulkPipes,
    slot: u8,
    dir_in: bool,
) -> XhciResult<()> {
    let (ring, dci) =
        if dir_in { (&pipes.in_ring, pipes.dci_in) } else { (&pipes.out_ring, pipes.dci_out) };
    let reset = reset_endpoint_command(slot, dci, cmd_ring.cycle() != 0);
    issue(doorbell_base, intr_base, cmd_ring, evt_ring, reset)?;
    let dequeue = ring.enqueue_phys();
    let set = set_dequeue_command(slot, dci, dequeue, ring.cycle() != 0, cmd_ring.cycle() != 0);
    issue(doorbell_base, intr_base, cmd_ring, evt_ring, set)
}
fn issue(
    doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
    trb: Trb,
) -> XhciResult<()> {
    let issued = cmd_ring.enqueue(trb)?;
    ring_doorbell(doorbell_base, 0, 0);
    wait_command_completion(intr_base, issued, evt_ring).map(|_| ())
}
