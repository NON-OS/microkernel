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

//! Decrypting the server's encrypted handshake records into one message run.

use alloc::vec::Vec;

use super::handshake_walk::walk;
use super::traffic_keys::TrafficKeys;

pub(super) fn handshake_messages(keys: &TrafficKeys, from: usize, bytes: &[u8]) -> Option<Vec<u8>> {
    walk(keys, from, bytes).map(|flight| flight.msgs)
}

pub(super) fn alert_in_flight(keys: &TrafficKeys, from: usize, bytes: &[u8]) -> Option<u8> {
    walk(keys, from, bytes).and_then(|flight| flight.alert)
}
