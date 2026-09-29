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
            tls_alert: None,
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
        }
    }

    /// End the fetch with `reason`.
    pub fn stop(&mut self, reason: &'static str) {
        self.error = Some(reason);
        self.phase = Phase::Error;
    }

    /// Whether the fetch has an outcome and nothing more to read.
    pub fn ended(&self) -> bool {
        matches!(self.phase, Phase::Decrypt | Phase::Done | Phase::Error)
    }
}
