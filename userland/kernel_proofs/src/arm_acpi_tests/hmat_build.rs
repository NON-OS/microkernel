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

//! HMAT structures as firmware lays them out (recalled from 5.2.28): the
//! attributes of a memory domain and a latency or bandwidth matrix.

use super::build::*;

fn structure(kind: u16, body: &[u8]) -> Vec<u8> {
    let len = 8 + body.len() as u32;
    [u16le(kind), u16le(0), u32le(len), body.to_vec()].concat()
}

pub fn attributes(initiator: Option<u32>, memory: u32) -> Vec<u8> {
    let flags = u16::from(initiator.is_some());
    let body = [u16le(flags), u16le(0), u32le(initiator.unwrap_or(0)), u32le(memory)];
    structure(0, &[body.concat(), vec![0; 20]].concat())
}

pub fn matrix(level: u8, kind: u8, base: u64, from: &[u32], to: &[u32], e: &[u16]) -> Vec<u8> {
    let head = [vec![level, kind, 0, 0], u32le(from.len() as u32), u32le(to.len() as u32)];
    let lists = [u32le(0), u64le(base), from.iter().flat_map(|d| u32le(*d)).collect()];
    let tail = [
        to.iter().flat_map(|d| u32le(*d)).collect(),
        e.iter().flat_map(|v| u16le(*v)).collect::<Vec<u8>>(),
    ];
    structure(1, &[head.concat(), lists.concat(), tail.concat()].concat())
}

pub fn hmat(structures: &[Vec<u8>]) -> Vec<u8> {
    sdt(b"HMAT", 2, &[u32le(0), structures.concat()].concat())
}
