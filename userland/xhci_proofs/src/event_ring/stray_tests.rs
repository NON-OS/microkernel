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

//! A command completion nobody waits for: the late answer to a command
//! that timed out, or the real completion behind a Ring Stopped event.
//! Commands run one at a time inside a request, so between requests, and
//! inside an interrupt-IN poll, none is in flight and such an event is
//! nobody's. Left at the head it held every interrupt-IN report behind it.

use super::events::{command, transfer, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT};
use super::producer::ERDP_EHB_BIT;
use crate::controller::{drain_events, poll_interrupt_in, IntrPoll};

const STRAY: u64 = 0xF0_DEAD_0000;

#[test]
fn a_command_completion_nobody_waits_for_does_not_hold_up_an_interrupt_report() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let armed = res.interrupt[0].ring.enqueue_phys();
    let first = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
    assert!(matches!(first, Ok(IntrPoll::Pending)), "armed, nothing yet");
    fx.hc.post(command(STRAY, CC_SUCCESS, SLOT));
    fx.hc.post(transfer(armed, CC_SUCCESS, 0, SLOT, INT_DCI));
    let mut got = Ok(IntrPoll::Pending);
    for _ in 0..2 {
        got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, 8);
        if matches!(got, Ok(IntrPoll::Complete(_))) {
            break;
        }
    }
    assert!(matches!(got, Ok(IntrPoll::Complete(8))), "the report behind it came through");
    assert_eq!(fx.hc.pending(), 0, "both handed back");
}

#[test]
fn the_idle_drain_consumes_a_command_completion_nobody_waits_for() {
    let mut fx = Fixture::new();
    fx.hc.post(command(STRAY, CC_SUCCESS, SLOT));
    let _ = drain_events(fx.intr(), &mut fx.ring);
    assert!(!fx.ring.has_event(), "nothing left at the head");
    assert_eq!(fx.hc.pending(), 0, "handed back through ERDP");
    assert_ne!(fx.hc.erdp() & ERDP_EHB_BIT, 0, "EHB written as one");
}

#[test]
fn the_idle_drain_still_leaves_a_transfer_event_for_its_poll() {
    let mut fx = Fixture::new();
    fx.hc.post(command(STRAY, CC_SUCCESS, SLOT));
    fx.hc.post(transfer(0x1000, CC_SUCCESS, 0, SLOT, INT_DCI));
    let _ = drain_events(fx.intr(), &mut fx.ring);
    assert_eq!(fx.hc.pending(), 1, "only the command completion was taken");
    assert!(fx.ring.has_event(), "the transfer event is at the head");
}
