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

//! What a circuit owns while it is built and once it carries traffic.

extern crate alloc;

use alloc::vec::Vec;

use crate::path::Relay;

use super::super::hop::Hop;
use super::stage::CircuitStage;

pub struct Circuit {
    pub id: u32,
    pub stage: CircuitStage,
    pub hops: Vec<Hop>,
    /// Guard first.
    pub path: Vec<Relay>,
    /// Counted separately from the window, not derived from it, so a window
    /// topped up early cannot suppress the next SENDME and stall the circuit.
    pub delivered_since: i32,
    /*
     * Open proves three relays answered a handshake. It does not prove the exit
     * carries traffic, and an exit that acks every control message while
     * delivering no payload is the failure the mixnet transport lost a session
     * to. Set only when a stream hands back a non-empty body.
     */
    pub proven: bool,
    /// Streams that ended with a failure reason on a circuit that has never
    /// delivered a byte. A broken exit answers control messages and then fails
    pub failures: u8,
    /// When the last hop came up, so a long session does not spend all of it
    /// behind one exit.
    pub opened_at: u64,
    /// The digest to acknowledge, pinned at the cell that brought the count due.
    /// `None` when nothing is owed. Held rather than read again at send time,
    pub owed_digest: Option<[u8; 20]>,
}
