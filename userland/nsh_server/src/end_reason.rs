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

//! Why a session ended, as the record's footer states it, and why an
//! escalation ended, as its `ESC_ENDED` entry states it
//! (docs/nsh/nsh-v1-record.md).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndReason {
    /// The client sent `CLOSE`.
    Close = 1,
    /// The shell exited on its own.
    Exit = 2,
    /// No input arrived within the idle timeout `AUTH_OK` stated.
    Idle = 3,
    /// The grant ran out, or the role's longest session was reached.
    Expired = 4,
    /// The session was aborted: a broken frame, a spent direction, or the
    /// client's `ABORT`.
    Aborted = 5,
    /// The host ended it: the grant was revoked, the record reached its
    /// cap, or the host could not keep the session.
    Host = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscEnd {
    /// The escalation grant's window closed.
    Expired = 1,
    /// The session it belonged to ended first.
    SessionEnded = 2,
    /// The escalation grant, or its base grant, was revoked.
    Revoked = 3,
}

impl EndReason {
    pub const fn code(self) -> u8 {
        self as u8
    }
}

impl EscEnd {
    pub const fn code(self) -> u8 {
        self as u8
    }
}
