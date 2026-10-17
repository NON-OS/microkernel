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

/*
 * Standard ABI words for the publisher's calls: static words, and one
 * trailing string (its offset, its length, its bytes zero-padded).
 */

use alloc::vec::Vec;

pub fn addr(a: &[u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(a);
    w
}

pub fn uint(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

/// A uint64 word read strictly: None when its top 24 bytes are not zero.
pub fn word_u64(w: &[u8; 32]) -> Option<u64> {
    let (high, low) = w.split_last_chunk::<8>()?;
    high.iter().all(|b| *b == 0).then(|| u64::from_be_bytes(*low))
}

/// selector | heads | (for a string) its offset word then its tail.
pub fn call(selector: u32, heads: &[[u8; 32]], string: Option<&[u8]>) -> Vec<u8> {
    let mut out = selector.to_be_bytes().to_vec();
    heads.iter().for_each(|w| out.extend_from_slice(w));
    if let Some(s) = string {
        out.extend_from_slice(&uint(32 * (heads.len() as u128 + 1)));
        out.extend_from_slice(&uint(s.len() as u128));
        out.extend_from_slice(s);
        out.resize(out.len() + (32 - s.len() % 32) % 32, 0);
    }
    out
}
