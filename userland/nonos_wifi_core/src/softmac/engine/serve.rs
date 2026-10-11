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

//! One request, and one pass between requests. net_core's link operations
//! are tried first, then the Wi-Fi control family, and a frame neither takes
//! is refused, so its caller never waits forever.

use super::dead::DeadPort;
use super::refuse::refuse;
use super::station::SoftStation;
use crate::netif;
use crate::softmac::radio::{Env, Radio};
use crate::softmac::session::Port;
use crate::softmac::stats::LinkStats;

impl<R: Radio, E: Env> SoftStation<R, E> {
    /// Answer `req` into `out` (at least `netif::MAX_RESPONSE` long);
    /// returns the reply length.
    pub fn handle(&mut self, req: &[u8], out: &mut [u8]) -> usize {
        let served = match self.radio.as_mut() {
            Some(radio) => {
                let session = self.session.as_mut();
                netif::serve(req, &mut Port { radio, session, stats: &mut self.stats }, out)
            }
            None => netif::serve(req, &mut DeadPort, out),
        };
        if let Some(n) = served {
            LinkStats::bump(&mut self.stats.netif_reqs);
            return n;
        }
        if let Some(n) = self.control(req, out) {
            return n;
        }
        refuse(req, out)
    }

    /// The work between requests: the chip's own upkeep, then the session's
    /// frames, or a step of the background scan while there is none.
    pub fn tick(&mut self) {
        let Some(radio) = self.radio.as_mut() else { return };
        radio.service(self.env.now_ms());
        match self.session.as_mut() {
            Some(s) => {
                if s.pump(radio, &mut self.env, &mut self.stats).is_some() {
                    self.end_session();
                }
            }
            None => self.scanner.step(radio, &self.env, self.domain),
        }
    }
}
