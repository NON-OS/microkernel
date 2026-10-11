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

//! Extension blocks built from CTA-861 section 7 and DisplayID 2.0.

use crate::build::seal;
pub use crate::build_did::{displayid, type7};

/// A CTA-861 revision 3 block: `blocks` is the data block collection,
/// header bytes included; `dtds` follow it. Byte 3 says basic audio.
pub fn cta(blocks: &[u8], dtds: &[[u8; 18]]) -> [u8; 128] {
    let mut b = [0u8; 128];
    b[0] = 0x02;
    b[1] = 3;
    b[2] = (4 + blocks.len()) as u8;
    b[3] = 0x40;
    b[4..4 + blocks.len()].copy_from_slice(blocks);
    let mut at = 4 + blocks.len();
    for d in dtds {
        b[at..at + 18].copy_from_slice(d);
        at += 18;
    }
    seal(&mut b);
    b
}

/// A data block: header byte (tag, length) and payload.
pub fn data(tag: u8, body: &[u8]) -> Vec<u8> {
    let mut v = vec![tag << 5 | body.len() as u8];
    v.extend_from_slice(body);
    v
}

/// An EDID of the base block with its count set, then `ext` in order.
pub fn with(mut base: [u8; 128], ext: &[[u8; 128]]) -> Vec<u8> {
    base[126] = ext.len() as u8;
    seal(&mut base);
    let mut v = base.to_vec();
    ext.iter().for_each(|e| v.extend_from_slice(e));
    v
}
