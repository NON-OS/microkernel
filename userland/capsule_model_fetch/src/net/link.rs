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

/*
 * A connection to a mirror as TLS sees it: bytes written, and bytes read
 * with zero meaning nothing has come yet. A link whose far end has closed
 * reads as an error once nothing is left.
 */

use nonos_socket::TcpStream;
use nonos_tls::{Io, SessionError};

use super::anon::AnonLink;
use super::socks::SocksLink;

pub enum Link {
    Direct(TcpStream),
    Nym(SocksLink),
    Anon(AnonLink),
}

impl Io for Link {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        match self {
            Link::Direct(s) => s.write_all(data).map_err(|_| SessionError::Io),
            Link::Nym(s) => s.write_all(data).map_err(|_| SessionError::Io),
            Link::Anon(s) => s.write_all(data).map_err(|_| SessionError::Io),
        }
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        match self {
            Link::Direct(s) => s.read(into).map_err(|_| SessionError::Io),
            Link::Nym(s) => s.read(into).map_err(|_| SessionError::Io),
            Link::Anon(s) => s.read(into).map_err(|_| SessionError::Io),
        }
    }
}
