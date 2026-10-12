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

//! An IORT as a server firmware writes one: an ITS group, an SMMUv3 whose
//! stream IDs map to the ITS, a root complex whose requester IDs map to
//! the SMMU, and a named component behind the SMMU.

use super::build::*;

pub const ITS: u32 = 48;
pub const SMMU: u32 = ITS + 24;
pub const RC: u32 = SMMU + 68 + 20;
pub const NAMED: u32 = RC + 36 + 20;

/// (input base, count less one, output base, output node, flags)
pub fn mapping(m: (u32, u32, u32, u32, u32)) -> Vec<u8> {
    [u32le(m.0), u32le(m.1), u32le(m.2), u32le(m.3), u32le(m.4)].concat()
}

pub fn node(ty: u8, rev: u8, id: u32, data: &[u8], maps: &[Vec<u8>]) -> Vec<u8> {
    let at = 16 + data.len();
    let len = (at + 20 * maps.len()) as u16;
    let head = [
        vec![ty],
        u16le(len),
        vec![rev],
        u32le(id),
        u32le(maps.len() as u32),
        u32le(if maps.is_empty() { 0 } else { at as u32 }),
    ];
    [head.concat(), data.to_vec(), maps.concat()].concat()
}

pub fn nodes() -> Vec<Vec<u8>> {
    let its = node(0, 1, 0, &[u32le(1), u32le(7)].concat(), &[]);
    let smmu_data = [
        u64le(0x4000_0000),
        u32le(1 << 4),
        u32le(0),
        u64le(0),
        u32le(0),
        u32le(200),
        u32le(201),
        u32le(202),
        u32le(203),
        u32le(0),
        u32le(0),
    ]
    .concat();
    let smmu = node(4, 4, 1, &smmu_data, &[mapping((0, 0xFFFF, 0x1_0000, ITS, 0))]);
    let rc_data = [u32le(1), vec![0; 4], u32le(1), u32le(0), vec![48, 0, 0, 0]].concat();
    let rc = node(2, 1, 2, &rc_data, &[mapping((0, 0xFF, 0x100, SMMU, 0))]);
    let named_data =
        [u32le(0), u32le(1), vec![0; 4], vec![44], b"\\_SB.ETH0\0".to_vec(), vec![0; 2]].concat();
    let named = node(1, 2, 3, &named_data, &[mapping((0, 0, 0x2000, SMMU, 1))]);
    vec![its, smmu, rc, named]
}

pub fn iort_of(nodes: &[Vec<u8>]) -> Vec<u8> {
    let head = [u32le(nodes.len() as u32), u32le(48), u32le(0)].concat();
    sdt(b"IORT", 5, &[head, nodes.concat()].concat())
}
