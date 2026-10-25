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

//! The connection on the wallet's route as the byte stream TLS reads. After
//! each write the far end may be a mixnet's seconds away, so the next read
//! waits the route's patience for a first byte; once bytes flow, a short
//! quiet wait ends the answer. A deadline bounds the whole exchange.

use nonos_libc::mk_uptime_ms;
use nonos_route_link::RouteStream;
use nonos_tls::{Io, SessionError};

/// The quiet wait once an answer has begun.
const FLOWING_MS: u64 = 2_000;

pub struct RouteIo {
    stream: RouteStream,
    patience_ms: u64,
    wait_ms: u64,
    deadline: i64,
}

impl RouteIo {
    pub fn new(stream: RouteStream, patience_ms: u64, total_ms: u64) -> RouteIo {
        let deadline = mk_uptime_ms().saturating_add(total_ms as i64);
        RouteIo { stream, patience_ms, wait_ms: patience_ms, deadline }
    }
}

impl Io for RouteIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.wait_ms = self.patience_ms;
        self.stream.write_all(data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        if mk_uptime_ms() > self.deadline {
            return Err(SessionError::Io);
        }
        if self.stream.ended() {
            return Ok(0);
        }
        let n = self.stream.read_wait(into, self.wait_ms).map_err(|_| SessionError::Io)?;
        if n > 0 {
            self.wait_ms = FLOWING_MS;
        }
        Ok(n)
    }
}
