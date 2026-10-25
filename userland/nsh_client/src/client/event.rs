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

//! What the client reports to its caller as frames arrive.

use alloc::vec::Vec;
use nsh_wire::{Reason, SESSION_ID_LEN};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoggedIn {
    pub session_id: [u8; SESSION_ID_LEN],
    pub idle_s: u32,
    /// The capabilities the session capsule was given, as names.
    pub caps: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The host passed every check; `USER_AUTH` is queued to send.
    HostAccepted,
    LoggedIn(LoggedIn),
    /// Terminal output.
    Output(Vec<u8>),
    /// The answer to an `ESCALATE`: `Reason::Ok` with the time it ends, or
    /// a refusal code.
    Escalated {
        reason: Reason,
        expires_ms: u64,
    },
    /// The shell exited with this status.
    Exit(i32),
    /// The whole session record, its parts joined. It is checked offline
    /// with `verify_record`.
    Record(Vec<u8>),
    /// The server closed the session.
    Closed,
}
