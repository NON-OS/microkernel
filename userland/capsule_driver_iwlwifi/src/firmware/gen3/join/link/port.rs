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
//! The link as net_core drives it, serviced between its requests.

use nonos_wifi_core::station::Tick;

use super::super::super::region::{Clock, Region};
use super::super::fw::{Fw, Queue};
use super::state::Link;
use crate::regs::Mmio;
/// The link as net_core drives it: the firmware and the open port.
pub struct Port<'p, 'a, 'd, M: Mmio, R: Region + ?Sized, C: Clock> {
    pub fw: &'p mut Fw<'a, 'd, M, R, C>,
    pub link: &'p mut Link,
}

impl<M: Mmio, R: Region + ?Sized, C: Clock> Port<'_, '_, '_, M, R, C> {
    /// Take what the firmware posted and answer what calls for it, then tick
    /// the station's clock: a probe or an SA Query to send, or the link's end.
    pub fn service(&mut self) {
        self.fw.pump();
        while let Some(rx) = self.fw.inbox.frames.pop_front() {
            if let Some(reply) = self.link.receive(rx) {
                self.send(&reply);
            }
        }
        if core::mem::take(&mut self.fw.inbox.missed) {
            self.link.station.beacons_missed();
        }
        match self.link.station.tick(self.fw.clock.now_ms()) {
            Tick::Send(frame) => self.send(&frame),
            Tick::Lost(why) => self.link.lost(why),
            Tick::Quiet => {}
        }
    }

    // A frame to the access point at the management rate: a management frame
    // (an SA Query, a probe) on the management queue, data on the data queue.
    fn send(&mut self, frame: &[u8]) {
        let queue =
            if frame.first().is_some_and(|b| b & 0x0C == 0) { Queue::Mgmt } else { Queue::Data };
        let _ = self.fw.send(queue, frame, self.link.mgmt_rate);
    }
}
