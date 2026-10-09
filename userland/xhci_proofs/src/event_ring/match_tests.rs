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

//! Which event completes which request.
//!
//! A command completes on the Command Completion Event whose pointer names
//! that command's TRB; a transfer completes on the Transfer Event whose
//! pointer names the TRB that asked for the interrupt and whose slot and
//! endpoint ids name the ring that TRB is on (xHCI 1.2 section 6.4.2.1).
//! Anything else that arrives first, whatever its completion code says, must
//! not finish the wait. Each test puts the impostors ahead of the real event
//! with a failing completion code or a different residual, so taking any of
//! them shows as an error or a wrong length rather than passing quietly.

use super::events::{command, port_change, transfer, CC_STALL, CC_SUCCESS, CC_TRB_ERROR};
use super::fixture::{Fixture, BULK_IN_DCI, EP0_DCI, INT_DCI, SLOT, TRB};
use crate::controller::wait_command_completion::wait_command_completion;
use crate::controller::{
    bulk_transfer, get_device_descriptor, issue_enable_slot, poll_interrupt_in, IntrPoll,
    DEVICE_DESCRIPTOR_LEN,
};
use crate::error::XhciError;
use crate::slots::SlotTable;
use crate::trb::commands::noop_command;
use crate::trb::Trb;

/// Pointers that are not `issued`: its neighbours on the ring, the same
/// address with the high half changed, the null pointer, and a page away.
fn not_issued(issued: u64) -> [u64; 5] {
    [issued + TRB, issued - TRB, issued ^ (1 << 40), 0, issued + 0x1000]
}

/// Slot and endpoint ids other than (`SLOT`, `dci`): slot 0, which is never
/// a device; slot 255, past any MaxSlots; a slot never enabled; endpoint 0,
/// which no DCI is; endpoint 31, the last; and another endpoint of the slot.
fn not_configured(dci: u8) -> [(u8, u8); 6] {
    let other = if dci == INT_DCI { EP0_DCI } else { INT_DCI };
    [(0, dci), (255, dci), (SLOT + 1, dci), (SLOT, 0), (SLOT, 31), (SLOT, other)]
}

#[test]
fn a_completion_naming_another_command_does_not_complete_this_one() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let _earlier = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    for (n, p) in not_issued(issued).into_iter().enumerate() {
        fx.hc.post(command(p, CC_TRB_ERROR, 10 + n as u8));
    }
    fx.hc.post(transfer(issued, CC_STALL, 0, SLOT, EP0_DCI));
    fx.hc.post(port_change(1));
    fx.hc.post(command(issued, CC_SUCCESS, 3));
    let done = wait_command_completion(fx.intr(), issued, &mut fx.ring).expect("completes");
    assert_eq!(done.slot_id, 3, "the completion that names the command");
    assert_eq!(fx.hc.pending(), 0, "every event ahead of it was read and handed back");
}

#[test]
fn a_command_answered_only_for_others_times_out() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    for p in not_issued(issued) {
        fx.hc.post(command(p, CC_SUCCESS, 1));
    }
    let got = wait_command_completion(fx.intr(), issued, &mut fx.ring);
    assert_eq!(got.map(|c| c.slot_id), Err(XhciError::CommandCompletionTimeout));
}

#[test]
fn reserved_low_pointer_bits_do_not_hide_the_command_they_name() {
    // Bits 3:0 of the pointer are RsvdZ, which software ignores on read.
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    fx.hc.post(command(issued | 0xF, CC_SUCCESS, 4));
    let done = wait_command_completion(fx.intr(), issued, &mut fx.ring).expect("completes");
    assert_eq!(done.slot_id, 4);
}

#[test]
fn an_enable_slot_answer_naming_no_usable_slot_is_refused_before_any_table_is_indexed() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let first = cmd.crcr_value() & !0x3F;
    fx.hc.post(command(first, CC_SUCCESS, 0));
    let got = issue_enable_slot(fx.db(), fx.intr(), &mut cmd, &mut fx.ring);
    assert_eq!(got, Err(XhciError::ControllerUnsupported), "slot 0 is never a device slot");

    fx.hc.post(command(first + TRB, CC_SUCCESS, 255));
    let slot = issue_enable_slot(fx.db(), fx.intr(), &mut cmd, &mut fx.ring).expect("answered");
    assert_eq!(slot, 255);
    let max_slots = 8;
    let mut table = SlotTable::new();
    assert!(!table.mark_allocated(slot, max_slots), "past MaxSlots, the table refuses it");
    assert!(!table.is_allocated(slot, max_slots));
    assert!(table.resources_mut(slot, max_slots).is_none());
    assert!(table.mark_allocated(3, max_slots), "a slot inside MaxSlots is taken");
}

#[test]
fn a_transfer_event_for_another_trb_does_not_complete_a_control_transfer() {
    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let setup = ep0.enqueue_phys();
    let status = setup + 2 * TRB;
    let out = fx.pool.alloc(64).expect("buffer");
    // None of these is one of the transfer's own three TRBs.
    for p in [status + TRB, status ^ (1 << 40), 0, setup.wrapping_sub(TRB)] {
        fx.hc.post(transfer(p, CC_STALL, 0, SLOT, EP0_DCI));
    }
    fx.hc.post(command(status, CC_TRB_ERROR, SLOT));
    fx.hc.post(transfer(status, CC_SUCCESS, 0, SLOT, EP0_DCI));
    let got = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
    assert_eq!(got, Ok(DEVICE_DESCRIPTOR_LEN as usize));
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn a_stall_on_the_data_stage_ends_the_control_transfer_at_once() {
    /*
     * The controller reports a STALL on the TRB it was on and halts EP0; the
     * status stage is never reached. The wait ends on that event with the
     * STALL, where it used to park it and run to its timeout.
     */
    for stage in [0, 1] {
        let mut fx = Fixture::new();
        let mut ep0 = fx.transfer_ring();
        let at = ep0.enqueue_phys() + stage * TRB;
        let out = fx.pool.alloc(64).expect("buffer");
        fx.hc.post(transfer(at, CC_STALL, 0, SLOT, EP0_DCI));
        let started = std::time::Instant::now();
        let got = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
        assert_eq!(got, Err(XhciError::TransferCompletionFailed(CC_STALL)), "stage {stage}");
        assert!(started.elapsed() < std::time::Duration::from_millis(500), "it ran to the timeout");
    }
}

#[test]
fn a_stall_on_another_slot_or_endpoint_does_not_end_a_control_transfer() {
    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let setup = ep0.enqueue_phys();
    let out = fx.pool.alloc(64).expect("buffer");
    fx.hc.post(transfer(setup + TRB, CC_STALL, 0, SLOT + 1, EP0_DCI));
    fx.hc.post(transfer(setup + TRB, CC_STALL, 0, SLOT, INT_DCI));
    fx.hc.post(transfer(setup + 2 * TRB, CC_SUCCESS, 0, SLOT, EP0_DCI));
    let got = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
    assert_eq!(got, Ok(DEVICE_DESCRIPTOR_LEN as usize));
}

#[test]
fn a_transfer_event_for_another_trb_does_not_complete_a_bulk_transfer() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let issued = pipes.in_ring.enqueue_phys();
    for p in not_issued(issued) {
        fx.hc.post(transfer(p, CC_STALL, 7, SLOT, BULK_IN_DCI));
    }
    fx.hc.post(command(issued, CC_TRB_ERROR, SLOT));
    fx.hc.post(transfer(issued, CC_SUCCESS, 100, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(412), "the residual of the event that names the TRB");
}

#[test]
fn a_transfer_event_for_another_trb_leaves_the_interrupt_poll_pending() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let issued = res.interrupt[0].ring.enqueue_phys();
    for p in not_issued(issued) {
        fx.hc.post(transfer(p, CC_SUCCESS, 0, SLOT, INT_DCI));
    }
    for _ in 0..not_issued(issued).len() {
        let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
        assert!(matches!(got, Ok(IntrPoll::Pending)));
    }
    assert_eq!(fx.hc.pending(), 0, "each was taken off the ring and kept for its owner");
    fx.hc.post(transfer(issued, CC_SUCCESS, 3, SLOT, INT_DCI));
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Complete(5))));
}

#[test]
fn the_interrupt_poll_leaves_a_command_completion_for_its_waiter() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let issued = res.interrupt[0].ring.enqueue_phys();
    fx.hc.post(command(issued, CC_SUCCESS, SLOT));
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Pending)));
    assert_eq!(fx.hc.pending(), 1, "not consumed");
    let head: Trb = fx.ring.current_trb();
    assert_eq!(head.get_type(), crate::constants::TRB_TYPE_CMD_COMPLETION_EVENT);
}

#[test]
fn a_transfer_event_for_a_slot_or_endpoint_never_configured_does_not_complete_a_control_transfer(
) {
    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let status = ep0.enqueue_phys() + 2 * TRB;
    let out = fx.pool.alloc(64).expect("buffer");
    for (slot, ep) in not_configured(EP0_DCI) {
        fx.hc.post(transfer(status, CC_STALL, 0, slot, ep));
    }
    fx.hc.post(transfer(status, CC_SUCCESS, 0, SLOT, EP0_DCI));
    let got = get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out);
    assert_eq!(got, Ok(DEVICE_DESCRIPTOR_LEN as usize));
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn a_transfer_event_for_a_slot_or_endpoint_never_configured_does_not_complete_a_bulk_transfer() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let issued = pipes.in_ring.enqueue_phys();
    for (slot, ep) in not_configured(BULK_IN_DCI) {
        fx.hc.post(transfer(issued, CC_STALL, 7, slot, ep));
    }
    fx.hc.post(transfer(issued, CC_SUCCESS, 100, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(412));
    // The impostors were parked. The next transfer on the ring is not handed
    // one of them either.
    let next = pipes.in_ring.enqueue_phys();
    fx.hc.post(transfer(next, CC_SUCCESS, 12, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(500));
}

#[test]
fn a_transfer_event_for_a_slot_or_endpoint_never_configured_leaves_the_interrupt_poll_pending() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let issued = res.interrupt[0].ring.enqueue_phys();
    for (slot, ep) in not_configured(INT_DCI) {
        fx.hc.post(transfer(issued, CC_SUCCESS, 1, slot, ep));
    }
    for (slot, ep) in not_configured(INT_DCI) {
        let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
        assert!(matches!(got, Ok(IntrPoll::Pending)), "slot {slot}, endpoint {ep}");
    }
    assert_eq!(fx.hc.pending(), 0, "each was taken off the ring");
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Pending)), "nor taken back out of the parked events");
    fx.hc.post(transfer(issued, CC_SUCCESS, 3, SLOT, INT_DCI));
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Complete(5))));
}
