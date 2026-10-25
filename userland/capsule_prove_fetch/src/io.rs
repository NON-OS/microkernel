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

//! The stream on the chosen route as TLS reads and writes it. Every read
//! and write first answers what has come in on fetch's endpoint, so STATUS
//! is answered while a transfer is under way, and a read waits at most a
//! slice for its first byte; the transfer keeps its own read budget. Past
//! the op's end every read and write fails, so the TLS handshake, which
//! waits on its own clock, is held to the op's deadline too.

use nonos_libc::mk_uptime_ms;
use nonos_route_link::RouteStream;
use nonos_tls::{Io, SessionError};

const SLICE_MS: u64 = 500;

pub struct RouteIo<'a> {
    pub stream: RouteStream,
    pub between: &'a mut dyn FnMut(),
    /// The op's end, in uptime ms.
    pub until: i64,
}

impl RouteIo<'_> {
    fn open(&mut self) -> Result<(), SessionError> {
        (self.between)();
        if mk_uptime_ms() >= self.until {
            return Err(SessionError::Io);
        }
        Ok(())
    }
}

impl Io for RouteIo<'_> {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.open()?;
        self.stream.write_all(data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        self.open()?;
        self.stream.read_wait(into, SLICE_MS).map_err(|_| SessionError::Io)
    }

    /// The route's patience: a proxied stream waits as its network asks
    /// (Proxy::patience_ms), a direct socket as the wire does.
    fn patience_ms(&self) -> i64 {
        match &self.stream {
            RouteStream::Proxied(t) => t.proxy().patience_ms() as i64,
            RouteStream::Direct(_) => 0,
        }
    }
}
