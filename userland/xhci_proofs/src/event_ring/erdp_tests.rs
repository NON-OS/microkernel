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

//! Handing events back through ERDP (xHCI 1.2 sections 4.9.4 and 5.5.2.3.3).
//!
//! ERDP carries the dequeue pointer in bits 63:4, the segment index (DESI) in
//! bits 2:0 and Event Handler Busy in bit 3. EHB is write-one-to-clear: the
//! controller sets it with IP and raises no further interrupt until software
//! writes a one there. Every hand-back after events are read must therefore
//! name the next unread TRB, inside the one segment, with DESI zero and EHB
//! one. The first programming, before any interrupt, leaves EHB alone.

use super::events::{command, numbered, port_change, transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, EP0_DCI, INT_DCI, SLOT, TRB};
use super::producer::{reg64, ERDP_EHB_BIT, ERDP_FLAGS};
use crate::constants::{ERSTBA_LO, IMAN, IMAN_IE};
use crate::controller::wait_command_completion::wait_command_completion;
use crate::controller::{
    bulk_transfer, drain_events, get_device_descriptor, poll_interrupt_in, IntrPoll,
};
use crate::trb::commands::noop_command;

fn assert_handed_back(fx: &Fixture) {
    let erdp = fx.hc.erdp();
    assert_eq!(erdp & !ERDP_FLAGS, fx.dequeue_bus(), "ERDP names the next unread TRB");
    assert_ne!(erdp & ERDP_EHB_BIT, 0, "EHB is written as one, which clears it");
    assert_eq!(erdp & 0x7, 0, "DESI names the only segment");
}

#[test]
fn programming_points_erdp_at_the_segment_base_and_leaves_ehb_alone() {
    let fx = Fixture::new();
    assert_eq!(fx.hc.erdp(), fx.hc.segment_bus().start, "no flags, segment start");
    assert_eq!(reg64(&fx.hc.regs, ERSTBA_LO), fx.ring.erst_base_phys());
    assert_eq!(fx.hc.segment_trbs(), 64);
    assert_ne!(fx.hc.regs.wrote32(IMAN as usize) & IMAN_IE, 0, "the interrupter is enabled");
}

#[test]
fn the_drain_hands_back_every_event_it_read() {
    let mut fx = Fixture::new();
    for port in 1..=5 {
        fx.hc.post(port_change(port));
    }
    let batch = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(batch.count, 5);
    assert_handed_back(&fx);
    assert_eq!(fx.hc.erdp() & !ERDP_FLAGS, fx.hc.segment_bus().start + 5 * TRB);
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn a_drain_with_nothing_to_read_still_clears_ehb() {
    let mut fx = Fixture::new();
    let batch = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(batch.count, 0);
    assert_handed_back(&fx);
}

#[test]
fn the_command_wait_hands_back_as_it_reads() {
    let mut fx = Fixture::new();
    let mut cmd = fx.command_ring();
    let issued = cmd.enqueue(noop_command(cmd.cycle() != 0)).expect("enqueue");
    fx.hc.post(port_change(1));
    fx.hc.post(command(issued, CC_SUCCESS, 0));
    wait_command_completion(fx.intr(), issued, &mut fx.ring).expect("completes");
    assert_handed_back(&fx);
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn the_control_transfer_wait_hands_back_as_it_reads() {
    let mut fx = Fixture::new();
    let mut ep0 = fx.transfer_ring();
    let status = ep0.enqueue_phys() + 2 * TRB;
    let out = fx.pool.alloc(64).expect("buffer");
    fx.hc.post(port_change(1));
    fx.hc.post(transfer(status, CC_SUCCESS, 0, SLOT, EP0_DCI));
    get_device_descriptor(fx.db(), fx.intr(), &mut fx.ring, SLOT, &mut ep0, &out)
        .expect("completes");
    assert_handed_back(&fx);
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn the_interrupt_poll_hands_back_what_it_takes() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let issued = res.interrupt[0].ring.enqueue_phys();
    fx.hc.post(transfer(issued, CC_SUCCESS, 0, SLOT, INT_DCI));
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Complete(8))));
    assert_handed_back(&fx);
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn the_bulk_wait_hands_back_as_it_reads() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    let issued = pipes.in_ring.enqueue_phys();
    fx.hc.post(port_change(1));
    fx.hc.post(transfer(issued, CC_SUCCESS, 0, SLOT, BULK_IN_DCI));
    let moved = bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, 512);
    assert_eq!(moved, Ok(512));
    assert_handed_back(&fx);
    assert_eq!(fx.hc.pending(), 0);
}

#[test]
fn the_hand_back_after_the_last_slot_names_the_segment_base() {
    let mut fx = Fixture::new();
    for n in 0..fx.hc.segment_trbs() {
        fx.hc.post(numbered(n as u32));
        assert_eq!(drain_events(fx.intr(), &mut fx.ring).count, 1);
        assert_handed_back(&fx);
    }
    assert_eq!(fx.hc.erdp(), fx.hc.segment_bus().start | ERDP_EHB_BIT);
}
