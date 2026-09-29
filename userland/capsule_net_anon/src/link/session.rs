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

//! An authenticated link to one relay, and the cells crossing it.

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use nonos_tls::stream::Stream;

use crate::cell::Frame;

use super::socket::Socket;

/// A link to a guard, after VERSIONS, CERTS and NETINFO.
pub struct Link {
    pub(super) socket: Socket,
    pub(super) stream: Stream,
    pub(super) partial: Vec<u8>,
    /*
     * Whole cells taken off the stream while something else was waiting for a
     * particular reply. One link carries every circuit, so a build waiting for its
     * CREATED2 also receives the cells of a circuit already carrying traffic.
     *
     * Dropping those would not merely lose payload. Each hop authenticates cells
     * against a running digest over everything it has received, so one cell
     * skipped puts that hop permanently out of step and every later cell on it
     * fails to verify: the circuit dies, and it looks like the far end broke
     * framing rather than like this side threw a cell away.
     */
    pub(super) held: VecDeque<Frame>,
}

/// Why a link could not be opened or has stopped working.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LinkError {
    Connect,
    Tls,
    Protocol,
    Version,
    Identity,
    Closed,
}
