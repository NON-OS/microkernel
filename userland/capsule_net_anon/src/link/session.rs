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

//! An authenticated link to one relay, and the cells crossing it.

extern crate alloc;

use alloc::vec::Vec;
use nonos_tls::stream::Stream;

use super::socket::Socket;
use super::tls12::Tls12Stream;

/// A link to a guard, after VERSIONS, CERTS and NETINFO.
pub struct Link {
    pub(super) socket: Socket,
    pub(super) stream: Tls,
    pub(super) partial: Vec<u8>,
}

/// The session under a link: TLS 1.3 from the shared nonos_tls, or TLS 1.2
/// from this capsule's own module for a relay that refused 1.3. The link
/// layer sees the same four calls either way, and binds either session's
/// certificate through CERTS the same way.
pub(super) enum Tls {
    V13(Stream),
    V12(Tls12Stream),
}

impl Tls {
    pub(super) fn write_all(&mut self, socket: &mut Socket, body: &[u8]) -> Result<(), ()> {
        match self {
            Self::V13(s) => s.write_all(socket, body).map_err(|_| ()),
            Self::V12(s) => s.write_all(socket, body).map_err(|_| ()),
        }
    }

    pub(super) fn read(&mut self, socket: &mut Socket) -> Result<Vec<u8>, ()> {
        match self {
            Self::V13(s) => s.read(socket).map_err(|_| ()),
            Self::V12(s) => s.read(socket).map_err(|_| ()),
        }
    }

    pub(super) fn is_done(&self) -> bool {
        match self {
            Self::V13(s) => s.is_done(),
            Self::V12(s) => s.is_done(),
        }
    }
}

/// Why a link could not be opened or has stopped working.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LinkError {
    Connect,
    Tls,
    Protocol,
    Version,
    Identity,
    Closed,
}
