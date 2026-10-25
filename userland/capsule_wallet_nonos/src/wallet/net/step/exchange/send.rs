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

//! What is due sent a slice at a time, then the gather for what answers;
//! and the request made due once the handshake is over.

use alloc::vec::Vec;

use nonos_tls::TrafficKeys;

use super::super::super::read_tls_flight::ANSWER_MAX;
use super::super::gather::Gather;
use super::said::NO_ANSWER;
use super::state::Exchange;
use super::types::{Next, Stage};

impl Exchange {
    pub(super) fn send(&mut self, next: Next) -> Result<Option<Vec<u8>>, &'static str> {
        let link = self.link.as_mut().ok_or(NO_ANSWER)?;
        self.sent += link.write_slice(&self.out[self.sent..]).map_err(|_| NO_ANSWER)?;
        if self.sent >= self.out.len() {
            let mut bounds = self.bounds();
            if matches!(next, Next::Answer) {
                bounds.max = ANSWER_MAX;
            }
            self.gather = Some(Gather::new(bounds, nonos_libc::mk_uptime_ms()));
            self.stage = match next {
                Next::Flight => Stage::Flight,
                Next::Answer => Stage::Answer,
            };
        }
        Ok(None)
    }

    /// The handshake is over: its state and the private keys go, and the
    /// request sealed behind the client's Finished is due.
    pub(super) fn sealed(&mut self, app: TrafficKeys, flight: Vec<u8>, end: usize) {
        self.handshake = None;
        if let Some(client) = self.flight.as_mut() {
            client.wipe_private();
        }
        self.app = Some(app);
        self.flight_end = end;
        self.out = flight;
        self.sent = 0;
        self.stage = Stage::Send { next: Next::Answer };
    }
}
