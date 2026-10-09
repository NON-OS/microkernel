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

//! The inputs the vectors were made from, regenerated. This must match
//! tools/nonos-zstd-vectors; a drift shows as every vector failing at once.

use crate::rng::Rng;

const WORDS: [&[u8]; 32] = [
    b"the", b"store", b"frame", b"kernel", b"package", b"of", b"a", b"link", b"guest", b"proof",
    b"and", b"block", b"window", b"signal", b"to", b"is", b"offset", b"literal", b"table",
    b"stream", b"in", b"trust", b"capsule", b"by", b"byte", b"zero", b"match", b"code", b"state",
    b"with", b"root", b"tar",
];

pub fn text(r: &mut Rng, n: usize) -> Vec<u8> {
    let mut out = Vec::new();
    let mut count = 0;
    while out.len() < n {
        out.extend_from_slice(WORDS[(r.next() >> 59) as usize]);
        count += 1;
        out.push(if count % 12 == 0 { b'\n' } else { b' ' });
    }
    out.truncate(n);
    out
}

pub fn noise(r: &mut Rng, n: usize) -> Vec<u8> {
    (0..n).map(|_| (r.next() >> 56) as u8).collect()
}

pub fn runs(r: &mut Rng, n: usize) -> Vec<u8> {
    let mut out = Vec::new();
    while out.len() < n {
        let byte = (r.next() >> 60) as u8;
        let len = (r.next() >> 58) as usize + 1;
        out.extend(std::iter::repeat_n(byte, len));
    }
    out.truncate(n);
    out
}

pub fn mixed(r: &mut Rng, n: usize) -> Vec<u8> {
    let mut out = Vec::new();
    while out.len() < n {
        let part = if (out.len() / 4096) % 2 == 0 { text(r, 4096) } else { noise(r, 4096) };
        out.extend(part);
    }
    out.truncate(n);
    out
}
