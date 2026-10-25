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

use super::traffic_keys::TrafficKeys;

/// The application traffic keys, given the hash of ClientHello..server Finished.
/// The master secret takes the handshake secret's place in the block.
pub fn app_keys(handshake: &TrafficKeys, th: &[u8; 32]) -> Option<TrafficKeys> {
    use super::schedule::{secret, stage, EMPTY_HASH};
    TrafficKeys::build(handshake.suite, |s| {
        let (zero, mut derived) = ([0u8; 32], [0u8; 32]);
        let ok = secret(&handshake.handshake_secret, b"derived", &EMPTY_HASH, &mut derived)
            && super::hkdf::extract(&derived, &zero, &mut s.handshake_secret);
        super::traffic_keys::wipe(&mut derived);
        ok && stage(s, th, b"c ap traffic", b"s ap traffic", handshake.suite)
    })
}
