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

//! What carries a stream outside the family, and how it ends.

use super::anon::Anon;

pub enum Backend {
    /// A net.sockets mixnet socket.
    Sockets(u32),
    /// A stream net.anon opened for this capsule.
    Anon(Anon),
}

/// Which service a call on the stream goes to, without the stream's state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Via {
    Sockets(u32),
    Anon,
}

/// A net.anon stream to close: the service's port and the stream's id.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Close {
    pub port: u32,
    pub id: u16,
}

/// How the far end finished, as a read reports it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum End {
    /// It finished cleanly: end of file.
    Eof,
    /// This errno, reported once, then end of file.
    Error(i64),
}

impl Backend {
    pub fn via(&self) -> Via {
        match self {
            Backend::Sockets(h) => Via::Sockets(*h),
            Backend::Anon(_) => Via::Anon,
        }
    }

    pub fn anon(&self) -> Option<&Anon> {
        match self {
            Backend::Anon(a) => Some(a),
            Backend::Sockets(_) => None,
        }
    }

    pub fn anon_mut(&mut self) -> Option<&mut Anon> {
        match self {
            Backend::Anon(a) => Some(a),
            Backend::Sockets(_) => None,
        }
    }
}
