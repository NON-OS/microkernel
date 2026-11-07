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

//! What net.sockets' answer to an accept means for the accept loop. Pure, so
//! nsh_front_proofs holds it.

/// net.sockets: nothing has finished its handshake yet.
pub const E_NO_TRANSPORT: u16 = 6;
/// net.sockets: a connection is ready and net.sockets has no room for it
/// now; it stays queued.
pub const E_TABLE_FULL: u16 = 7;
const E_OK: u16 = 0;

/// The result of an accept: a new child socket, nothing waiting, or the
/// listener is gone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accept {
    Child(u32),
    Pending,
    Gone,
}

/// The loop's reading of an answer: `None` when net.sockets did not answer
/// (or answered a frame that does not parse), else its errno and, when the
/// payload holds one, the child's handle. Every other errno is the listener
/// gone, which the loop reopens: net.sockets says so once net.tcp no longer
/// holds the listener (the route left Direct, or net.core restarted).
pub fn accepted(answer: Option<(u16, Option<u32>)>) -> Accept {
    match answer {
        Some((E_OK, Some(child))) => Accept::Child(child),
        Some((E_NO_TRANSPORT | E_TABLE_FULL, _)) => Accept::Pending,
        _ => Accept::Gone,
    }
}
