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

//! A Unix socket's name, who a message came from, and a datagram.

use alloc::vec::Vec;

use super::types::Addr;

/// A bound Unix name. `key` is what two sockets must share to meet: the
/// resolved path, or a NUL then the abstract name. `shown` is the sun_path
/// the guest gave, which getsockname and accept report back.
#[derive(Clone, PartialEq, Eq)]
pub struct UName {
    pub key: Vec<u8>,
    pub shown: Vec<u8>,
}

impl UName {
    /// True for a name in the abstract namespace, which no file backs.
    pub fn is_abstract(&self) -> bool {
        self.key.first() == Some(&0)
    }
}

/// The far end of a message or a connection, as a sockaddr reports it: an
/// IPv4 address, or a Unix name, None for an unnamed socket.
#[derive(Clone)]
pub enum Peer {
    Inet(Addr),
    Unix(Option<UName>),
}

/// A datagram waiting to be read, with its sender.
pub struct Gram {
    pub from: Peer,
    pub bytes: Vec<u8>,
}
