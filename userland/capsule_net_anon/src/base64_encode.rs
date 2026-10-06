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

//! Base64 without padding, for the digests a microdescriptor request names.

extern crate alloc;

use alloc::vec::Vec;

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/*
 * Unpadded, because that is how a consensus writes a microdescriptor digest and
 * how the DirPort expects it back. Sending the '=' padding gives a URL the
 * authority does not recognise, and the request comes back empty rather than
 * refused, which is the harder failure to read.
 */
/// `data` as unpadded base64.
pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len().div_ceil(3) * 4);
    for group in data.chunks(3) {
        let mut acc = 0u32;
        for (index, byte) in group.iter().enumerate() {
            acc |= (*byte as u32) << (16 - 8 * index);
        }
        let symbols = group.len() + 1;
        for index in 0..symbols {
            let shift = 18 - 6 * index;
            out.push(ALPHABET[((acc >> shift) & 0x3f) as usize]);
        }
    }
    out
}
