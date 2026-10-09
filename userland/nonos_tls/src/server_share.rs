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

//! The ECDHE share a ServerHello carries, in one of the two groups offered.

use super::constants::{GROUP_SECP256R1, GROUP_X25519};

/// The server's share, in the group it chose from the two this client sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServerShare {
    X25519([u8; 32]),
    /// The uncompressed point: 0x04, then x, then y.
    Secp256r1([u8; 65]),
}

/*
 * One group, its key length, the key. A length that is not the group's, a
 * compressed or hybrid P-256 point, or a group this client did not offer is
 * refused.
 */
pub fn parse(body: &[u8]) -> Option<ServerShare> {
    let group = super::read::u16_at(body, 0)?;
    let len = super::read::u16_at(body, 2)? as usize;
    let key = super::read::slice(body, 4, len)?;
    match (group, len) {
        (GROUP_X25519, 32) => {
            let mut out = [0u8; 32];
            out.copy_from_slice(key);
            Some(ServerShare::X25519(out))
        }
        (GROUP_SECP256R1, 65) if key.first() == Some(&4) => {
            let mut out = [0u8; 65];
            out.copy_from_slice(key);
            Some(ServerShare::Secp256r1(out))
        }
        _ => None,
    }
}
