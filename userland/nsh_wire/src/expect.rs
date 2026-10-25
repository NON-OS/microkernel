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

//! What one side may receive next. The receiver holds an `Expect`; a frame
//! of a known kind that it does not admit is refused as `Order`, and a
//! length over its cap is refused before the body is read.

use crate::limits::{AUTH_CAP, HELLO_CAP, SESSION_CAP};
use crate::Kind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expect {
    /// The server, before anything: `CLIENT_HELLO`.
    ClientHello,
    /// The client, after its hello: `SERVER_HELLO`, or `ABORT(VERSION)`.
    ServerHello,
    /// The client, after `SERVER_HELLO`: `HOST_PROOF`.
    HostProof,
    /// The server, after `HOST_PROOF`: `USER_AUTH`, or `ABORT(POLICY)`.
    UserAuth,
    /// The client, after `USER_AUTH`: `AUTH_OK` or `AUTH_REFUSED`.
    AuthResult,
    /// The server while the session runs.
    ServerSession,
    /// The client while the session runs.
    ClientSession,
}

impl Expect {
    /// The hellos travel in plaintext; every frame after them is sealed.
    pub const fn sealed(self) -> bool {
        !matches!(self, Expect::ClientHello | Expect::ServerHello)
    }

    pub const fn cap(self) -> usize {
        match self {
            Expect::ClientHello | Expect::ServerHello => HELLO_CAP,
            Expect::HostProof | Expect::UserAuth | Expect::AuthResult => AUTH_CAP,
            Expect::ServerSession | Expect::ClientSession => SESSION_CAP,
        }
    }

    pub const fn admits(self, kind: Kind) -> bool {
        use Kind::*;
        match self {
            Expect::ClientHello => matches!(kind, ClientHello),
            Expect::ServerHello => matches!(kind, ServerHello | Abort),
            Expect::HostProof => matches!(kind, HostProof | Abort),
            Expect::UserAuth => matches!(kind, UserAuth | Abort),
            Expect::AuthResult => matches!(kind, AuthOk | AuthRefused | Abort),
            Expect::ServerSession => {
                matches!(kind, Input | Window | Signal | Escalate | Close | Abort)
            }
            Expect::ClientSession => {
                matches!(kind, Output | EscalateResult | Exit | Record | Close | Abort)
            }
        }
    }
}
