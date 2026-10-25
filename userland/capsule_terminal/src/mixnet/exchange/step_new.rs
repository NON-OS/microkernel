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

//! Making an exchange, and what it says of where it has got to.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_libc::mk_time_millis;
use nonos_route_link::Route;

use super::super::types::{After, Exchange, Phase, Stage};
use super::super::wait::Wait;

impl Exchange {
    /// `request` is the whole HTTP request; `limit` bounds what the far end
    /// can make this allocate.
    pub fn new(
        route: Route,
        host: &str,
        port: u16,
        secure: bool,
        rtc: u64,
        request: Vec<u8>,
        limit: usize,
    ) -> Self {
        Exchange {
            host: String::from(host),
            port,
            route,
            anonymous: route.is_anonymous(),
            secure,
            rtc,
            request,
            limit,
            phase: Phase::Start,
            stream: None,
            out: Vec::new(),
            sent: 0,
            raw: Vec::new(),
            tls: None,
            wait: Wait::new(mk_time_millis(), route.is_anonymous()),
        }
    }

    pub fn stage(&self) -> Stage {
        match self.phase {
            Phase::Start | Phase::Opening(_) => Stage::Connecting,
            Phase::Send(After::Flight) | Phase::Flight => Stage::Handshake,
            Phase::Send(After::Response) => Stage::Sending,
            Phase::Response | Phase::Done => Stage::Receiving,
        }
    }

    /// Bytes of the flight or response that have arrived so far.
    pub fn received(&self) -> usize {
        self.raw.len()
    }
}
