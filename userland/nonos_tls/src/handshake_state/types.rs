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

//! One connection's handshake, from the ServerHello on.

use alloc::vec::Vec;

use crate::traffic_keys::TrafficKeys;
use crate::transcript::Transcript;

/*
 * Derived once, when the ServerHello is whole, and kept. Every question asked
 * of the flight afterwards (has it finished, did the server stop with an
 * alert, does it verify) reads this instead of redoing the key agreement and
 * decrypting the flight again from its first record.
 */
pub struct HandshakeState {
    pub(crate) keys: TrafficKeys,
    /// ClientHello and ServerHello, hashed.
    pub(crate) transcript: Transcript,
    /// Wire offset of the first record not yet taken.
    pub(crate) cursor: usize,
    /// Sequence number of the next encrypted record.
    pub(crate) seq: u64,
    /// The decrypted handshake messages, in order.
    pub(crate) msgs: Vec<u8>,
    /// Offset in `msgs` of the first message not yet looked at.
    pub(crate) scanned: usize,
    pub(crate) end: Option<usize>,
    pub(crate) alert: Option<u8>,
    pub(crate) broken: bool,
}

/// How far the server's flight has come.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Progress {
    /// Needs more bytes before it can be judged.
    Incomplete,
    /// A whole server Finished has been decrypted; the flight ends at this
    /// wire offset and anything after it belongs to the application.
    Complete(usize),
    /// The server stopped with this alert instead of finishing.
    Alert(u8),
    /// A record would not open under the handshake keys.
    Broken,
}

/// What the bytes so far allow before the handshake keys exist.
pub enum Start {
    /// The ServerHello record is not whole yet.
    Waiting,
    Ready(HandshakeState),
    /// The server refused in the clear, before any ServerHello.
    Alert(u8),
    /// The server asked for a second ClientHello with another key share.
    Retry,
    /// A ServerHello that cannot be keyed: wrong suite, version or share.
    Unusable,
}
