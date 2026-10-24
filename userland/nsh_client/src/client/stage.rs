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

//! Where the client is, and what it may receive there. In the session the
//! server's ending order is kept: `EXIT`, then the record's parts, then
//! `CLOSE`; output stops at `EXIT`, and an `ESCALATE_RESULT` answers only
//! an `ESCALATE` the client sent.

use nsh_wire::Expect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stage {
    ServerHello,
    HostProof,
    AuthResult,
    Session(Phase),
    Over,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Ending {
    Running,
    Exited,
    Record,
    RecordDone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Phase {
    pub ending: Ending,
    /// An `ESCALATE` was sent and has not been answered.
    pub escalating: bool,
    /// The client sent `CLOSE`, and sends nothing more.
    pub closing: bool,
}

impl Phase {
    pub(crate) const START: Phase =
        Phase { ending: Ending::Running, escalating: false, closing: false };
}

impl Stage {
    /// What the client may receive now; nothing once the session is over.
    pub(crate) const fn expect(self) -> Option<Expect> {
        match self {
            Stage::ServerHello => Some(Expect::ServerHello),
            Stage::HostProof => Some(Expect::HostProof),
            Stage::AuthResult => Some(Expect::AuthResult),
            Stage::Session(_) => Some(Expect::ClientSession),
            Stage::Over => None,
        }
    }

    /// What the server is waiting for, which sets the cap of what the
    /// client sends it.
    pub(crate) const fn peer(self) -> Expect {
        match self {
            Stage::Session(_) => Expect::ServerSession,
            _ => Expect::UserAuth,
        }
    }
}
