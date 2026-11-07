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

//! What one read of a connection means for the session loop. Pure, so
//! nsh_front_proofs holds it. net.core answers an empty read with an error
//! whether the client is only quiet or has gone, and net.sockets passes both
//! on as E_NO_TRANSPORT; the connection's POLL bits tell the two apart.

use crate::accepted::E_NO_TRANSPORT;

const E_OK: u16 = 0;
/// net.core's POLL: the peer has finished and nothing is left to read.
pub const POLL_FINISHED: u8 = 0x04;

/// A read's meaning: bytes, nothing yet, or the connection is over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    Data,
    Idle,
    Gone,
}

/// `answer` is net.sockets' errno and payload length for the read, `None`
/// when it did not answer. `poll` asks for the connection's POLL bits (its
/// errno and the bits when the answer held them) and is called only for an
/// empty read. Only a live connection whose peer has not finished is quiet.
pub fn read_is(
    answer: Option<(u16, usize)>,
    poll: impl FnOnce() -> Option<(u16, Option<u8>)>,
) -> Read {
    match answer {
        Some((E_OK, n)) if n > 0 => Read::Data,
        Some((E_OK, _)) => Read::Idle,
        Some((E_NO_TRANSPORT, _)) => match poll() {
            Some((E_OK, Some(bits))) if bits & POLL_FINISHED == 0 => Read::Idle,
            _ => Read::Gone,
        },
        _ => Read::Gone,
    }
}
