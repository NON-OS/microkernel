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

//! A command that never completes is aborted (xHCI 1.2 section 4.6.1.2),
//! and the Command Aborted and Command Ring Stopped events that follows
//! answer no later command. The Ring Stopped event names the TRB the next
//! command is written to; taken as its completion, a Disable Slot the
//! controller carried out was reported failed, the slot stayed in the
//! table, and the next Enable Slot handed out that same id was refused.

use super::bench::Bench;
use super::hc_device::lock;
use crate::controller::{issue_address_device, issue_disable_slot, issue_enable_slot};
use crate::error::XhciError;
use crate::protocol::{E_IO, OP_ADDRESS_DEVICE, OP_DISABLE_SLOT, OP_ENABLE_SLOT};

const HIGH_SPEED: u32 = 3;

#[test]
fn an_address_device_that_times_out_leaves_the_slot_table_in_step_with_the_controller() {
    let mut b = Bench::new(HIGH_SPEED);
    let slot = b.enable_slot();
    lock(&b.hc).stall_address = Some(slot);
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[slot, 1]).status, E_IO, "SET_ADDRESS never finished");
    let disabled = b.call(OP_DISABLE_SLOT, &[slot]);
    assert_eq!(disabled.status, 0, "Disable Slot reports what the controller did");
    assert!(!lock(&b.hc).enabled[slot as usize], "the controller disabled it");
    // The controller hands out the lowest free slot: the same one again.
    let again = b.call(OP_ENABLE_SLOT, &[]);
    assert_eq!(again.status, 0, "the table let the slot go with the controller");
    assert_eq!(again.body[0], slot);
}

#[test]
fn a_command_given_up_does_not_answer_the_next_one() {
    let mut b = Bench::new(HIGH_SPEED);
    let hc = b.hc.clone();
    let d = &mut b.mux.ctxs[0].driver;
    let (db, intr) = (d.layout.doorbell_base, d.layout.primary_intr_base);
    let slot = issue_enable_slot(db, intr, &mut d.command_ring, &mut d.event_ring).expect("slot");
    lock(&hc).stall_address = Some(slot);
    let input = d.dma_pool.alloc(4096).expect("input context");
    let (cmd, evt) = (&mut d.command_ring, &mut d.event_ring);
    let got = issue_address_device(db, intr, cmd, evt, input.phys(), slot);
    assert_eq!(got, Err(XhciError::CommandCompletionTimeout));
    while lock(&hc).stuck.is_some() {
        std::thread::yield_now();
    }
    assert!(!evt.has_event(), "Command Aborted and Command Ring Stopped were consumed");
    assert_eq!(issue_disable_slot(db, intr, cmd, evt, slot), Ok(()));
}
