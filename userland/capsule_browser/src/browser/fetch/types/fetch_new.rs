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

//! A fetch as it starts, and as it stops.

use alloc::vec::Vec;

use super::{Fetch, Phase};
use crate::browser::net::mixnet::Way;
use crate::browser::url::Url;

impl Fetch {
    /// A fetch of `url` on `handle`, starting in `phase` at `now_ms`.
    pub fn new(url: Url, handle: u32, phase: Phase, now_ms: i64) -> Fetch {
        Fetch {
            url,
            handle,
            phase,
            buf: Vec::new(),
            socks: Vec::new(),
            tls: None,
            dial: None,
            started_ms: now_ms,
            progress_ms: now_ms,
            received: 0,
            error: None,
            stopped_in: None,
            tls_alert: None,
            tls_why: None,
            suppress: false,
            image: None,
            hops: 0,
            post: None,
            js_req: false,
            css: false,
            font: 0,
            script: false,
            order: 0,
            rx_consumed: 0,
            tx_seq: 0,
            keep_uses: 0,
            keep: false,
            requested: false,
            truncated: false,
            way: Way::Direct,
            hold: None,
            not_before: 0,
            silent_base: None,
        }
    }

    /// The network the request's bytes ride, whose cookie jar it reads and
    /// fills. It is the way the connection opened with, and nothing else:
    /// an .anyone host is Anyone whatever the reader chose, a page whose
    /// route is off opens nothing and so touches no jar, and a request on a
    /// kept connection or a parallel stream rides that connection's way.
    pub fn net(&self) -> crate::browser::net::mixnet::Network {
        self.way.network()
    }

    /// End the fetch with `reason`.
    pub fn stop(&mut self, reason: &'static str) {
        if self.phase != Phase::Error {
            self.stopped_in = Some(self.phase);
        }
        self.error = Some(reason);
        self.phase = Phase::Error;
    }

    /// Whether the fetch has an outcome and nothing more to read.
    pub fn ended(&self) -> bool {
        matches!(self.phase, Phase::Decrypt | Phase::Done | Phase::Error)
    }
}
