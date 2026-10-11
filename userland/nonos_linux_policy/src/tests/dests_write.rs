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

//! Writing destination lists for the tests: one entry, and a list of them
//! in the order their bytes sort.

pub fn entry(kind: u8, prefix: u8, ports: (u16, u16), body: &[u8]) -> Vec<u8> {
    let mut e = vec![kind, prefix];
    e.extend_from_slice(&ports.0.to_be_bytes());
    e.extend_from_slice(&ports.1.to_be_bytes());
    e.push(u8::try_from(body.len()).unwrap());
    e.extend_from_slice(body);
    e
}

/// `entries` sorted, as a canonical list must be.
pub fn list(mut entries: Vec<Vec<u8>>) -> Vec<u8> {
    entries.sort();
    raw(&entries)
}

/// `entries` in the order given.
pub fn raw(entries: &[Vec<u8>]) -> Vec<u8> {
    let mut out = u16::try_from(entries.len()).unwrap().to_be_bytes().to_vec();
    entries.iter().for_each(|e| out.extend_from_slice(e));
    out
}

pub fn v4(prefix: u8, ports: (u16, u16), a: [u8; 4]) -> Vec<u8> {
    entry(4, prefix, ports, &a)
}

pub fn name(n: &str, ports: (u16, u16)) -> Vec<u8> {
    entry(b'N', 0, ports, n.as_bytes())
}
