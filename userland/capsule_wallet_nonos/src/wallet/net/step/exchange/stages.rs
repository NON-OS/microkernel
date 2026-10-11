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

//! The exchange's first stages: the link opened, and the hello built with
//! the handshake's one end set.

use alloc::vec::Vec;

use super::super::super::handshake_end::end_at;
use super::super::super::probe_rpc_tcp::TcpProbe;
use super::said::NO_HELLO;
use super::state::Exchange;
use super::types::{Next, Stage};

impl Exchange {
    pub(super) fn open(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let Stage::Open(opening) = &mut self.stage else { return Ok(None) };
        match opening.step() {
            Ok(None) => Ok(None),
            Ok(Some(link)) => {
                self.tcp = TcpProbe { resolve: true, socket: true, connect: true };
                self.link = Some(link);
                self.stage = Stage::Hello;
                Ok(None)
            }
            Err(reach) => {
                self.tcp =
                    TcpProbe { resolve: reach.resolve, socket: reach.socket, connect: false };
                self.unopened = Some(reach.why);
                Err(reach.why)
            }
        }
    }

    pub(super) fn hello(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let now = nonos_libc::mk_uptime_ms().max(0) as u64;
        self.handshake_end = Some(end_at(now));
        let flight = nonos_tls::client_flight(self.host.as_bytes()).ok_or(NO_HELLO)?;
        self.out = flight.record.clone();
        self.flight = Some(flight);
        self.sent = 0;
        self.stage = Stage::Send { next: Next::Flight };
        Ok(None)
    }
}
