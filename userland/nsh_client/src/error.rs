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

//! Why the client ended a session, would not start one, or would not send.
//! Each refusal is typed so the caller can show it by name; the server is
//! told no more than the one reason code the wire allows, and after a host
//! refusal it is told only `POLICY`.

use crate::{HostRefusal, Mismatch};
use nsh_wire::{Reason, WireError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// A frame broke the wire format or arrived out of order. Once keys
    /// exist the server is told the error's reason code.
    Wire(WireError),
    /// A sealed frame did not open. Nothing is answered.
    Open,
    /// A direction is spent. The session closes without a frame.
    Limit,
    /// The host was refused. The server is told `POLICY`.
    Host(HostRefusal),
    /// The server refused the login, for this reason.
    AuthRefused(Reason),
    /// The server ended the session, for this reason.
    Aborted(Reason),
    /// The record is not this session's. The server is told `PROTOCOL`.
    Record(Mismatch),
    /// The record would pass its 64 MiB cap. The server is told `LIMIT`.
    RecordLimit,
    /// The user's key did not sign. The session closes without a frame.
    Sign,
    /// The caller asked to send something the server would refuse: a grant
    /// that is not the right kind, or a window out of range. Nothing was
    /// sent, and the session goes on.
    Unsendable(WireError),
    /// The grant is not for the user key the client holds.
    GrantSubject,
    /// That call is not one the session allows now. Nothing was sent.
    NotNow,
    /// The session is over.
    Ended,
}
