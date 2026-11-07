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

//! Why the server refused a connection, a login or an escalation, or ended
//! a session. The host logs the whole `Why`; the client is told no more
//! than the one reason code the wire allows.

use crate::AttestFault;
use nsh_wire::{Reason, WireError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Why {
    /// A frame broke the wire format or came out of order.
    Wire(WireError),
    /// A sealed frame did not open. Nothing is answered.
    Open,
    /// A direction is spent. The session closes without a frame.
    Limit,
    /// The handshake ran past its deadline.
    HandshakeTime,
    /// The kernel could not attest. The client is told `BUSY`.
    Attest(AttestFault),
    /// The host keys did not sign. Nothing is answered.
    Sign,
    /// The entropy source failed. Nothing is answered.
    Rand,
    /// The revocation list is not one the role table trusts; every login
    /// is refused `REVOCATION_STALE` until a trusted one is installed.
    Policy(nsh_roles::PolicyError),
    /// The login's grant, as `nsh_roles` refused it.
    Login(nsh_roles::Refusal),
    /// An escalation's grant, as `nsh_roles` refused it. The session goes on.
    Escalation(nsh_roles::Refusal),
    /// The host could not start the session or the escalation it admitted.
    Denied,
    /// The client ended the session with this reason.
    Aborted(Reason),
}

impl Why {
    /// The `ABORT` code this refusal is told as once keys exist, if any.
    pub(crate) const fn abort(self) -> Option<Reason> {
        match self {
            Why::Wire(e) => Some(e.reason()),
            Why::HandshakeTime => Some(Reason::Limit),
            Why::Attest(_) | Why::Denied => Some(Reason::Busy),
            Why::Open | Why::Limit | Why::Sign | Why::Rand | Why::Aborted(_) => None,
            Why::Policy(_) | Why::Login(_) | Why::Escalation(_) => Some(Reason::Protocol),
        }
    }
}
