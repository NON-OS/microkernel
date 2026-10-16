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

//! The interrupt-IN poll as the fuzz rounds run it; a bulk and a control
//! transfer are `waits_transfer`.

use super::driver::Driver;
use super::genuine::genuine;
use super::rng::Rng;
use crate::controller::{poll_interrupt_in, IntrPoll};
use crate::error::XhciError;
use crate::event_ring::fixture::INT_DCI;
use crate::event_ring::fixture_slots::REPORT;

impl Driver {
    pub(super) fn poll(&mut self, rng: &mut Rng) {
        let length = rng.below(REPORT as u64 + 1) as u16;
        let ep = &self.hid.interrupt[0];
        let issued = ep.armed.unwrap_or(ep.ring.enqueue_phys());
        self.noise(rng, issued, INT_DCI, length as u32);
        if rng.coin() {
            let event = genuine(rng, issued, INT_DCI, length as u32);
            self.fx.hc.post(event);
        }
        let got = poll_interrupt_in(
            self.fx.db(),
            self.fx.intr(),
            &mut self.fx.ring,
            &mut self.hid,
            INT_DCI,
            length,
        );
        match got {
            Ok(IntrPoll::Complete(n)) => {
                assert!(n <= length, "{n} bytes handed up for {length}");
                assert!(n <= REPORT);
            }
            Ok(IntrPoll::Pending) => {}
            // A report that ended in an error code is reported as that error
            // and disarms the endpoint, which the server then recovers.
            Err(XhciError::TransferCompletionFailed(_)) => {
                assert!(self.hid.interrupt[0].armed.is_none(), "a failed report stays armed");
            }
            Err(e) => panic!("the poll failed: {e:?}"),
        }
    }
}
