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

//! The ring's consumers that wait on a transfer of their own besides the
//! interrupt poll: a bulk transfer and a control transfer.

use super::driver::{Driver, RING_TRBS};
use super::genuine::genuine;
use super::rng::Rng;
use crate::controller::{bulk_transfer, get_device_descriptor, DEVICE_DESCRIPTOR_LEN};
use crate::error::XhciError;
use crate::event_ring::fixture::{BULK_IN_DCI, EP0_DCI, SLOT, TRB};
use crate::protocol::BULK_MAX;

impl Driver {
    pub(super) fn bulk(&mut self, rng: &mut Rng) {
        let len = rng.below(BULK_MAX as u64) as u32 + 1;
        let issued = self.pipes.in_ring.enqueue_phys();
        self.noise(rng, issued, BULK_IN_DCI, len);
        let event = genuine(rng, issued, BULK_IN_DCI, len);
        self.fx.hc.post(event);
        let moved = bulk_transfer(
            self.fx.db(),
            self.fx.intr(),
            &mut self.fx.ring,
            &mut self.pipes,
            SLOT,
            true,
            len,
        );
        match moved {
            Ok(n) => assert!(n <= len && n as usize <= BULK_MAX, "{n} bytes moved for {len}"),
            Err(XhciError::TransferCompletionFailed(_)) => {}
            Err(e) => panic!("the bulk transfer failed: {e:?}"),
        }
    }

    pub(super) fn control(&mut self, rng: &mut Rng) {
        let at = (self.ep0.enqueue_phys() - self.ep0.phys()) / TRB;
        let status = self.ep0.phys() + ((at + 2) % RING_TRBS) * TRB;
        self.noise(rng, status, EP0_DCI, DEVICE_DESCRIPTOR_LEN as u32);
        let event = genuine(rng, status, EP0_DCI, DEVICE_DESCRIPTOR_LEN as u32);
        self.fx.hc.post(event);
        let got = get_device_descriptor(
            self.fx.db(),
            self.fx.intr(),
            &mut self.fx.ring,
            SLOT,
            &mut self.ep0,
            &self.desc,
        );
        match got {
            // A Short Packet the noise put on the data stage takes its
            // residual off; never more than was asked for.
            Ok(n) => assert!(n <= DEVICE_DESCRIPTOR_LEN as usize, "{n} bytes for 18"),
            Err(XhciError::TransferCompletionFailed(_)) => {}
            Err(e) => panic!("the control transfer failed: {e:?}"),
        }
    }
}
