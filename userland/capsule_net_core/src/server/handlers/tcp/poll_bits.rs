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

//! The readiness bits POLL answers for one connection, from its state alone:
//! readable, writable, and finished. A RECV with nothing to hand back answers
//! E_RX_EMPTY whether the peer is quiet or gone, so FINISHED is how a reader
//! tells the end of a stream from a pause in it.

use smoltcp::socket::tcp::State;

use crate::protocol::tcp::{POLL_FINISHED, POLL_READABLE, POLL_WRITABLE};

/// `state`, `can_recv` and `can_send` as the socket reports them.
/// FINISHED is set once the peer's FIN has arrived (or the connection is
/// closed) and every byte before it has been read.
pub fn poll_bits(state: State, can_recv: bool, can_send: bool) -> u8 {
    let mut b = 0u8;
    if can_recv {
        b |= POLL_READABLE;
    }
    if can_send {
        b |= POLL_WRITABLE;
    }
    if !can_recv && peer_finished(state) {
        b |= POLL_FINISHED;
    }
    b
}

/// The states in which the peer will send nothing more.
fn peer_finished(state: State) -> bool {
    matches!(
        state,
        State::CloseWait | State::LastAck | State::Closing | State::TimeWait | State::Closed
    )
}
