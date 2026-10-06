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

//! The version a hop reads out of the header to know how we built it.

use super::fields::VERSION_LENGTH;

/// Versions are a big endian u16 behind a leading zero. The spare byte is
/// left over from an older reading of the field and is not part of the
/// number, so a version written into the first byte names no version at all.
const fn version_bytes(value: u16) -> [u8; VERSION_LENGTH] {
    let b = value.to_be_bytes();
    [0, b[0], b[1]]
}

/// Payload key seeds over standard X25519: the version the reference client
/// and every node on the network now speak (sphinx-packet 0.6).
///
/// This says how a hop derives the key that unwraps its own payload layer:
/// from a 16-byte seed it stretches with HKDF-SHA256, where 258 had it read
/// the 192-byte key straight from its shared secret. Every hop reads this
/// field, so a version nobody else sends marks each of our packets as ours.
/// It also lets a reply block carry one seed per hop instead of one key, which
/// takes a request with its blocks from eighteen packets to seven.
pub const PACKET_VERSION: [u8; VERSION_LENGTH] = version_bytes(SEEDS_VERSION);

/// The first version whose hops stretch a seed into their payload key.
pub const SEEDS_VERSION: u16 = 259;

/// Whether a hop of a header written with `version` stretches a seed.
pub const fn uses_key_seeds(version: [u8; VERSION_LENGTH]) -> bool {
    u16::from_be_bytes([version[1], version[2]]) >= SEEDS_VERSION
}
