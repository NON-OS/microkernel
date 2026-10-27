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

//! The model host, made to flood: once on, it fills its own ring with
//! packets of its own each time the driver makes room, and a millisecond
//! passes at each look at the clock.

use nonos_vmbus::model::HostRing;
use nonos_vmbus::msg::MESSAGE_MAX;
use nonos_vmbus::packet::PACKET_DATA_INBAND;
use nonos_vmbus::{Host, PostError};

use crate::constants::layout::PAGE;
use crate::model::StorHost;

/// The model host, flooding the guest's ring once `on`.
pub struct Flood(pub StorHost, pub bool, pub u32);

impl Flood {
    /// Fill the host's ring with packets of the host's own until no room.
    fn fill(&mut self) {
        let Some((_, gpadl, at)) = self.0.vmbus.open.filter(|_| self.1) else { return };
        self.2 += 1;
        assert!(self.2 < 200_000, "the flood outlasted every deadline");
        let n = self.0.vmbus.gpadls.iter().find(|(id, _)| *id == gpadl).unwrap().1.len();
        let ring = HostRing { hdr: at as usize * PAGE, len: ((n - at as usize - 1) * PAGE) as u32 };
        let mut m = self.0.grant.pages(n * PAGE);
        while ring.send(&mut m, PACKET_DATA_INBAND, 0, &[0u8; 64]).is_some() {}
    }
}

impl Host for Flood {
    fn post(&mut self, m: &[u8]) -> Result<(), PostError> {
        self.0.post(m)
    }
    fn answer(&mut self, out: &mut [u8; MESSAGE_MAX]) -> Option<usize> {
        self.0.answer(out)
    }
    fn signal(&mut self) {
        self.0.signal();
        self.fill();
    }
    fn now_ms(&mut self) -> u64 {
        // Time passes as the packets come: a millisecond each look.
        self.0.vmbus.now += self.1 as u64;
        self.0.now_ms()
    }
    fn idle(&mut self) {
        self.0.idle();
        self.fill();
    }
}
