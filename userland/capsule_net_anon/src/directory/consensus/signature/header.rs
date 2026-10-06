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

//! Reading the signature line, whose field positions shift.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::lines::args;

/*
 * `directory-signature [algname] <identity> <signing-key-digest>`
 *
 * The algorithm is optional and its absence means sha1, so the two fingerprints
 * sit at different positions depending on whether it is there. Reading them at a
 * fixed index attributes a signature to the wrong authority, which is worse than
 * failing to read it: it would count toward the quorum under a name it does not
 * belong to.
 */
/// The algorithm flag and the two fingerprints, or `None` if the line does not
/// have two or three fields or a fingerprint is not forty hex characters.
pub fn parse_header(rest: &[u8]) -> Option<(bool, [u8; 20], [u8; 20])> {
    let fields: Vec<&[u8]> = args(rest).collect();
    let (sha256, identity, signing) = match fields.as_slice() {
        [identity, signing] => (false, *identity, *signing),
        [algorithm, identity, signing] => (*algorithm == b"sha256", *identity, *signing),
        _ => return None,
    };
    Some((sha256, hex20(identity)?, hex20(signing)?))
}

fn hex20(text: &[u8]) -> Option<[u8; 20]> {
    if text.len() != 40 {
        return None;
    }
    let mut out = [0u8; 20];
    for (index, slot) in out.iter_mut().enumerate() {
        *slot = (nibble(text[index * 2])? << 4) | nibble(text[index * 2 + 1])?;
    }
    Some(out)
}

fn nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}
