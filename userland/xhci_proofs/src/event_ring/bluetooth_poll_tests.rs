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

//! The event endpoint and the bulk IN pipe polled side by side on one
//! slot, as an HCI transport does: a look with nothing yet answers Pending
//! and arms one TRB, a look after the device answered hands up its length.

use super::bluetooth_tests::{radio_slot, EVENT_PACKET};
use super::bulk_poll_tests::LEN;
use super::events::{transfer, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, INT_DCI, SLOT, TRB};
use crate::controller::{poll_bulk_in, poll_interrupt_in, BulkPoll, IntrPoll};

#[test]
fn a_16_octet_event_packet_comes_up_whole() {
    let mut fx = Fixture::new();
    let mut res = radio_slot(&fx);
    let armed = res.interrupt[0].ring.enqueue_phys();
    fx.hc.post(transfer(armed, CC_SUCCESS, 0, SLOT, INT_DCI));
    let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, EVENT_PACKET);
    assert!(matches!(got, Ok(IntrPoll::Complete(EVENT_PACKET))));
    assert_eq!(res.interrupt[0].max, EVENT_PACKET, "the buffer holds one packet");
}

#[test]
fn an_empty_look_then_one_with_data_on_both_endpoints() {
    let mut fx = Fixture::new();
    let mut res = radio_slot(&fx);
    let event_at = res.interrupt[0].ring.enqueue_phys();
    let acl_at = res.bulk.as_ref().expect("bulk pair").in_ring.enqueue_phys();
    let look = |fx: &mut Fixture, res: &mut _| {
        poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, res, INT_DCI, EVENT_PACKET)
    };
    assert!(matches!(look(&mut fx, &mut res), Ok(IntrPoll::Pending)));
    let pipes = res.bulk.as_mut().expect("bulk pair");
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, pipes, SLOT, LEN);
    assert_eq!(got, Ok(BulkPoll::Pending));
    assert_eq!(res.interrupt[0].armed, Some(event_at), "one TRB armed, kept for the next look");
    // The device answers both; each look takes its own completion.
    fx.hc.post(transfer(event_at, CC_SUCCESS, 10, SLOT, INT_DCI));
    fx.hc.post(transfer(acl_at, CC_SUCCESS, LEN - 27, SLOT, BULK_IN_DCI));
    assert!(
        matches!(look(&mut fx, &mut res), Ok(IntrPoll::Complete(6))),
        "16 less a residual of 10"
    );
    let pipes = res.bulk.as_mut().expect("bulk pair");
    let got = poll_bulk_in(fx.db(), fx.intr(), &mut fx.ring, pipes, SLOT, LEN);
    assert_eq!(got, Ok(BulkPoll::Complete(27)));
    // The next looks are empty again and arm the following TRBs.
    assert!(matches!(look(&mut fx, &mut res), Ok(IntrPoll::Pending)));
    assert_eq!(res.interrupt[0].armed, Some(event_at + TRB));
}
