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

//! The session as net_core sees it: an Ethernet interface. Frames go out
//! through the station's framing and CCMP onto the radio, except while the
//! station is away on a look, when they are dropped; frames the pump
//! queued come back in order.

use super::state::Session;
use crate::netif::LinkPort;
use crate::softmac::radio::Radio;
use crate::softmac::stats::LinkStats;

pub struct Port<'a, R: Radio> {
    pub radio: &'a mut R,
    pub session: Option<&'a mut Session>,
    pub stats: &'a mut LinkStats,
    /// The station is off the session's channel: nothing goes out.
    pub closed: bool,
}

impl<R: Radio> LinkPort for Port<'_, R> {
    fn mac(&self) -> Option<[u8; 6]> {
        // The session's own, which its frames name, whatever the chip holds.
        self.session.as_ref().filter(|s| s.is_up()).map(|s| s.mac())
    }

    fn link_up(&self) -> bool {
        self.session.as_ref().is_some_and(|s| s.is_up())
    }

    fn poll_rx(&mut self, out: &mut [u8]) -> Option<usize> {
        let s = self.session.as_mut()?;
        let f = s.queue.pop_front()?;
        let n = f.len().min(out.len());
        out[..n].copy_from_slice(&f[..n]);
        Some(n)
    }

    fn send_tx(&mut self, frame: &[u8]) -> bool {
        let sent = match self.session.as_mut().filter(|_| !self.closed) {
            Some(s) => s.station.tx_frame(frame).is_some_and(|mpdu| self.radio.send(&mpdu)),
            None => false,
        };
        LinkStats::bump(if sent { &mut self.stats.tx_ok } else { &mut self.stats.tx_drop });
        sent
    }
}
