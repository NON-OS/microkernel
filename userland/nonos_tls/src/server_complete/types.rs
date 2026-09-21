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

//! What a completed server flight yields.

extern crate alloc;

use alloc::vec::Vec;

use crate::traffic_keys::TrafficKeys;

pub struct ServerComplete {
    pub handshake: TrafficKeys,
    pub app: TrafficKeys,
    pub transcript: Vec<u8>,
    /// The Certificate message body, so a caller that authenticates the peer
    /// itself can reach the leaf. Empty if none arrived.
    pub certificates: Vec<u8>,
}
