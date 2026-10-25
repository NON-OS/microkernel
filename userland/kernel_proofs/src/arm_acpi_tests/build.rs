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

//! Tables built as firmware writes them: the 36-byte header, the body,
//! the length and the checksum filled in last.

pub fn le<T: Copy + Into<u64>>(v: T, n: usize) -> Vec<u8> {
    v.into().to_le_bytes()[..n].to_vec()
}

pub fn u16le(v: u16) -> Vec<u8> {
    le(v, 2)
}

pub fn u32le(v: u32) -> Vec<u8> {
    le(v, 4)
}

pub fn u64le(v: u64) -> Vec<u8> {
    le(v, 8)
}

/// The length and checksum set to what the bytes now are.
pub fn seal(t: &mut [u8]) {
    let len = t.len() as u32;
    t[4..8].copy_from_slice(&len.to_le_bytes());
    t[9] = 0;
    let sum = t.iter().fold(0u8, |s, b| s.wrapping_add(*b));
    t[9] = 0u8.wrapping_sub(sum);
}

pub fn sdt(signature: &[u8; 4], revision: u8, body: &[u8]) -> Vec<u8> {
    let mut t = signature.to_vec();
    t.extend_from_slice(&[0; 4]);
    t.push(revision);
    t.push(0);
    t.extend_from_slice(b"NONOS TESTTBL   ");
    t.extend_from_slice(&[0; 12]);
    t.truncate(36);
    t.extend_from_slice(body);
    seal(&mut t);
    t
}

/// A structure: type, an 8-bit length, then the rest.
pub fn structure(ty: u8, rest: &[u8]) -> Vec<u8> {
    [vec![ty, (rest.len() + 2) as u8], rest.to_vec()].concat()
}
