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

//! The connection on the chosen route as the byte stream TLS reads and
//! writes. Through an anonymity network the first byte of an answer is
//! seconds away, so a read waits up to `PATIENCE_MS` before it says nothing
//! came; the download's own idle limit (job.rs) decides when that is too long.

use nonos_route_link::RouteStream;
use nonos_tls::{Io, SessionError};

const PATIENCE_MS: u64 = 2_000;

pub struct RouteIo(pub RouteStream);

impl Io for RouteIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.0.write_all(data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        self.0.read_wait(into, PATIENCE_MS).map_err(|_| SessionError::Io)
    }
}
