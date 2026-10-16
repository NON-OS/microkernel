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

//! The residual length in a Transfer Event (bits 23:0 of its third dword)
//! counts the bytes of the TRB that did not move. The controller writes it,
//! so it is anything up to 0xFF_FFFF. An interrupt report is armed at the
//! endpoint's whole payload, so a long report never babbles; what moved is
//! that less the residual clamped to it, and the bytes handed up are at
//! most what the client asked for: never past its request or the buffer.
//! A residual past the payload is `residual_wild_tests`, the bulk residual
//! `residual_bulk_tests`.

use super::events::{transfer, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT};
use super::fixture_slots::REPORT;
use crate::controller::{poll_interrupt_in, IntrPoll};
use crate::slots::SlotResources;

/// The third dword of a successful event reporting `residual`.
pub(super) fn success(residual: u32) -> u32 {
    (CC_SUCCESS as u32) << 24 | residual
}

/// Post an event with third dword `d2` for the interrupt-IN TRB the next poll
/// arms, poll once, and return the length handed up.
pub(super) fn report(fx: &mut Fixture, res: &mut SlotResources, length: u16, d2: u32) -> u16 {
    let issued = res.interrupt[0].ring.enqueue_phys();
    let mut event = transfer(issued, CC_SUCCESS, 0, SLOT, INT_DCI);
    event.d2 = d2;
    fx.hc.post(event);
    match poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, res, INT_DCI, length) {
        Ok(IntrPoll::Complete(n)) => {
            assert!(n <= length, "{n} bytes handed up for a {length}-byte request");
            assert!(n <= REPORT, "{n} bytes is past the report buffer");
            n
        }
        Ok(IntrPoll::Pending) => panic!("the event naming the armed TRB did not complete it"),
        Err(e) => panic!("the poll failed: {e:?}"),
    }
}

#[test]
fn an_interrupt_report_is_the_payload_less_the_residual_and_at_most_the_request() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    let payload = REPORT as u32;
    for length in [1u16, 4, REPORT] {
        for r in [0, 1, payload - 1, payload] {
            let n = report(&mut fx, &mut res, length, success(r));
            assert_eq!(n as u32, (payload - r).min(length as u32), "length {length}, residual {r}");
        }
    }
}
