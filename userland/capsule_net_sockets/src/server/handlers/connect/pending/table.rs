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

//! The connects in flight, each with the caller its reply belongs to.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use crate::sockets::SocketKey;

/*
 * A connect used to hold the serve loop until its handshake resolved, polling
 * the state and yielding for up to eight seconds, so every other caller of
 * this service waited behind one slow peer. It is now recorded here and the
 * loop goes back to serving; the reply goes out when the handshake resolves.
 * The kernel matches a reply to its caller by pid, so answering later is
 * answering the same call.
 */
pub(super) struct Pending {
    pub pid: u32,
    pub op: u16,
    pub request_id: u32,
    pub key: SocketKey,
    pub transport: u32,
    pub ip: [u8; 4],
    pub port: u16,
    pub deadline_ms: i64,
}

pub(super) static PENDING: Mutex<Vec<Pending>> = Mutex::new(Vec::new());

/// Whether any connect is still waiting, so the loop knows to come back.
pub fn waiting() -> bool {
    !PENDING.lock().is_empty()
}
