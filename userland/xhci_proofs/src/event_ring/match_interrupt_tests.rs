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

//! Which event completes an interrupt-IN poll: the Transfer Event whose
//! pointer, slot and endpoint name the armed TRB.

use super::events::{command, transfer, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT};
use super::match_impostors::{not_configured, not_issued};
use crate::controller::{poll_interrupt_in, IntrPoll};

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
fn the_interrupt_poll_takes_a_command_completion_nobody_waits_for() {
    // No command is in flight inside a poll (`stray_tests`): left at the
    // head, such an event held every report behind it.
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let issued = res.interrupt[0].ring.enqueue_phys();
    fx.hc.post(command(issued, CC_SUCCESS, SLOT));
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(got, Ok(IntrPoll::Pending)), "it completes no report");
    assert_eq!(fx.hc.pending(), 0, "consumed and handed back");
    assert!(!fx.ring.has_event(), "nothing left at the head");
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
