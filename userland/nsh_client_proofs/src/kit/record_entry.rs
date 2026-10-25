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

//! A record entry, `seq
//!  at_ms
//!  kind
//!  body`, and the sampled entries
//! a good record is made of.

use super::put16;
use nonos_hash::{sha256, sha512};

pub fn entry(seq: u32, at_ms: u64, kind: u8, body: &[u8]) -> Vec<u8> {
    let mut out = seq.to_le_bytes().to_vec();
    out.extend_from_slice(&at_ms.to_le_bytes());
    out.push(kind);
    put16(&mut out, body);
    out
}

/// Entry `seq`'s kind and body: commands, output digests, refused calls,
/// command digests and refused escalations in turn.
pub fn sample(seq: u32) -> (u8, Vec<u8>) {
    let n = seq.to_le_bytes();
    match seq % 5 {
        0 => (1, [&b"ls /var/log/"[..], &n].concat()),
        1 => (2, [&4096u64.to_le_bytes()[..], &sha256(&n)].concat()),
        2 => (3, [&n[..], &[7], b"Network"].concat()),
        3 => (8, sha256(&n).to_vec()),
        _ => (5, [&33u16.to_le_bytes()[..], &sha512(&n)].concat()),
    }
}
