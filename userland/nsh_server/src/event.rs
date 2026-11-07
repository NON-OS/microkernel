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

//! What the server reports to the host as frames arrive and time passes.

use crate::{Admission, EndReason, Why};
use alloc::vec::Vec;
use nsh_roles::CapSet;
use nsh_wire::{Signal, SESSION_ID_LEN};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The handshake was refused, a frame broke the session, or an
    /// escalation was refused. The host logs it.
    Refused(Why),
    /// A login passed every check; the host must answer it.
    Admit(Admission),
    /// `AUTH_OK` was sent: the session runs under this id.
    Started {
        session_id: [u8; SESSION_ID_LEN],
        caps: CapSet,
    },
    /// Terminal input for the shell.
    Input(Vec<u8>),
    Window {
        cols: u16,
        rows: u16,
    },
    Signal(Signal),
    /// An escalation passed every check; the host must start the second
    /// session capsule with `caps` until `ends_ms`, and answer
    /// `Server::escalation_begun` or `Server::escalation_failed`.
    Escalate {
        caps: CapSet,
        ends_ms: u64,
    },
    /// The running escalation ended; the host stops its capsule.
    EscalationEnded,
    /// The shell must stop. The host stops it and calls `Server::exit`.
    Stop(EndReason),
    /// Bytes appended to the record, in order: the host keeps them, so a
    /// session cut short by a crash still has its record up to the last
    /// checkpoint.
    Record(Vec<u8>),
    /// A checkpoint, `nsh-ckpt/1`, over the record so far.
    Checkpoint(Vec<u8>),
    /// The session is over, for this reason; nothing more is sent. The
    /// host closes the connection once it has sent the last output.
    Closed(EndReason),
}
