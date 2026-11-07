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

//! Where the server is, and what it may receive there.

use nsh_wire::Expect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stage {
    /// Nothing has arrived: `CLIENT_HELLO` is awaited.
    ClientHello,
    /// `HOST_PROOF` is sent: `USER_AUTH` is awaited.
    UserAuth,
    /// The login passed; the host is starting the session capsule. A
    /// client frame now is out of order: the client waits for `AUTH_OK`.
    Admitting,
    /// `AUTH_OK` is sent and the session runs.
    Session,
    Over,
}

impl Stage {
    /// What the server may receive now; nothing once it is over.
    pub(crate) const fn receives(self) -> Option<Expect> {
        match self {
            Stage::ClientHello => Some(Expect::ClientHello),
            Stage::UserAuth | Stage::Admitting => Some(Expect::UserAuth),
            Stage::Session => Some(Expect::ServerSession),
            Stage::Over => None,
        }
    }

    /// What the client is waiting for, which sets the cap of what the
    /// server sends it.
    pub(crate) const fn peer(self) -> Expect {
        match self {
            Stage::ClientHello => Expect::ServerHello,
            Stage::UserAuth | Stage::Admitting => Expect::AuthResult,
            Stage::Session | Stage::Over => Expect::ClientSession,
        }
    }
}
