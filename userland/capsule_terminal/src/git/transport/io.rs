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
//! Bridging the connection to what TLS expects.

use nonos_route_link::RouteStream;
use nonos_tls::{Io, SessionError};

/// The connection on the chosen route, seen as the byte stream a TLS session
/// reads and writes.
///
/// TLS gives up on a server that stays quiet for a few seconds, which is a
/// direct socket's sense of time. Through an anonymity network the first
/// byte of every answer is many seconds out, so a read waits up to the
/// route's patience for bytes before it says none came; on a direct
/// connection that patience is zero and a read is the one socket read it
/// always was. A far end that has finished reads as nothing more, so TLS
/// ends the response on its own quiet window as it did before.
pub(super) struct SocketIo {
    pub(super) stream: RouteStream,
    pub(super) patience_ms: u64,
}

impl Io for SocketIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        self.stream.write_all(data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        self.stream.read_wait(into, self.patience_ms).map_err(|_| SessionError::Io)
    }
}
