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

//! A residual past the interrupt report's payload, which no controller
//! that moved bytes reports, hands up nothing: a stale buffer is never
//! taken for a report.

use super::events::{transfer, CC_SUCCESS};
use super::fixture::{Fixture, INT_DCI, SLOT};
use super::fixture_slots::REPORT;
use super::residual_tests::{report, success};
use crate::controller::poll_interrupt_in;

#[test]
fn a_residual_past_the_payload_hands_up_nothing() {
    let mut fx = Fixture::new();
    let mut res = fx.hid_slot();
    for length in [1u16, 4, REPORT] {
        for r in [REPORT as u32 + 1, 0xFFFF, 0x00FF_FFFF] {
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
