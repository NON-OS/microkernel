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

//! A net.tcp handle dressed as the byte stream nonos_tls drives.

use nonos_tls::{Io, SessionError};

use crate::tcp_client::{close, connect, recv, send_all, wait_established};

/// One connection to a relay's ORPort. Closes on drop so a failed handshake
/// does not leave net.tcp holding a socket nobody will read.
pub struct Socket {
    port: u32,
    handle: u32,
}

impl Socket {
    /// Connect and wait for the three way handshake. `None` if net.tcp refuses
    /// or the relay never completes it.
    pub fn open(port: u32, address: [u8; 4], or_port: u16) -> Option<Self> {
        let handle = connect(port, address, or_port).ok()?;
        let socket = Self { port, handle };
        wait_established(port, handle).ok()?;
        Some(socket)
    }
}

impl Io for Socket {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        send_all(self.port, self.handle, data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        recv(self.port, self.handle, into).map_err(|_| SessionError::Io)
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        let _ = close(self.port, self.handle);
    }
}
