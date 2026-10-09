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
//! counts the requested bytes that did not move. The controller writes it,
//! so it is anything up to 0xFF_FFFF. The length handed up is the request
//! less the residual clamped to the request: never more than was asked for,
//! so never more than the buffer the request was sized to.

use super::events::{transfer, CC_SHORT_PACKET, CC_SUCCESS};
use super::fixture::{Fixture, BULK_IN_DCI, INT_DCI, SLOT};
use crate::controller::{bulk_transfer, poll_interrupt_in, IntrPoll};
use crate::protocol::{BULK_MAX, HID_REPORT_MAX};
use crate::slots::SlotResources;

/// The third dword of a successful event reporting `residual`.
fn success(residual: u32) -> u32 {
    (CC_SUCCESS as u32) << 24 | residual
}

/// Post an event with third dword `d2` for the interrupt-IN TRB the next poll
/// arms, poll once, and return the length handed up.
fn report(fx: &mut Fixture, res: &mut SlotResources, length: u16, d2: u32) -> u16 {
    let issued = res.interrupt[0].ring.enqueue_phys();
    let mut event = transfer(issued, CC_SUCCESS, 0, SLOT, INT_DCI);
    event.d2 = d2;
    fx.hc.post(event);
    match poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, res, INT_DCI, length) {
        Ok(IntrPoll::Complete(n)) => {
            assert!(n <= length, "{n} bytes handed up for a {length}-byte request");
            assert!(n as usize <= HID_REPORT_MAX, "{n} bytes is past the report buffer");
            n
        }
        Ok(IntrPoll::Pending) => panic!("the event naming the armed TRB did not complete it"),
        Err(e) => panic!("the poll failed: {e:?}"),
    }
}

#[test]
fn an_interrupt_report_is_the_request_less_the_residual() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    for length in [1u16, 4, HID_REPORT_MAX as u16] {
        let len = length as u32;
        for r in [0, 1, len - 1, len] {
            let n = report(&mut fx, &mut res, length, success(r));
            assert_eq!(n as u32, len - r, "length {length}, residual {r}");
        }
    }
}

#[test]
fn a_residual_past_the_request_hands_up_nothing() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    for length in [1u16, 4, HID_REPORT_MAX as u16] {
        for r in [length as u32 + 1, 0xFFFF, 0x00FF_FFFF] {
            let n = report(&mut fx, &mut res, length, success(r));
            assert_eq!(n, 0, "length {length}, residual {r:#x}: a stale buffer was handed up");
        }
        // Every bit set: completion code 0xFF over the widest residual. That
        // code is no success, so the poll reports the error and hands up
        // nothing at all.
        let issued = res.interrupt[0].ring.enqueue_phys();
        let mut event = transfer(issued, CC_SUCCESS, 0, SLOT, INT_DCI);
        event.d2 = u32::MAX;
        fx.hc.post(event);
        let got = poll_interrupt_in(fx.db(), fx.intr(), &mut fx.ring, &mut res, INT_DCI, length);
        assert!(
            matches!(got, Err(crate::error::XhciError::TransferCompletionFailed(0xFF))),
            "length {length}, third dword all ones"
        );
    }
}

#[test]
fn a_bulk_transfer_moves_the_request_less_the_residual_clamped_to_it() {
    let mut fx = Fixture::new();
    let mut pipes = fx.bulk_pipes();
    for len in [1u32, 512, BULK_MAX as u32] {
        for r in [0, 1, len - 1, len, len + 1, 0xFFFF, 0x00FF_FFFF] {
            for code in [CC_SUCCESS, CC_SHORT_PACKET] {
                let issued = pipes.in_ring.enqueue_phys();
                let mut event = transfer(issued, code, 0, SLOT, BULK_IN_DCI);
                event.d2 = (code as u32) << 24 | r;
                fx.hc.post(event);
                let moved =
                    bulk_transfer(fx.db(), fx.intr(), &mut fx.ring, &mut pipes, SLOT, true, len);
                assert_eq!(moved, Ok(len - r.min(len)), "length {len}, residual {r:#x}");
            }
        }
    }
}
