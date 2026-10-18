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

//! An IORT as an Arm laptop's firmware might write one: an ITS group, an
//! SMMUv2 with its global, context and PMU interrupts mapping its stream
//! IDs to the ITS, a root complex behind the SMMU, and a reserved memory
//! range node naming two stream ranges on the SMMU and one on the ITS.

use super::build::*;
use super::iort_build::{mapping, node};

pub const ITS: u32 = 48;
pub const SMMU: u32 = ITS + 24;
pub const RC: u32 = SMMU + 128;
pub const RMR: u32 = RC + 56;

/// GSIV and flags, eight bytes.
fn irq(gsiv: u32, edge: bool) -> Vec<u8> {
    [u32le(gsiv), u32le(u32::from(edge))].concat()
}

pub fn smmu_v2(context_at: u32) -> Vec<u8> {
    let fixed = [u64le(0x1500_0000), u64le(0x8_0000), u32le(1), u32le(1)].concat();
    let refs = [u32le(60), u32le(3), u32le(context_at), u32le(1), u32le(100)].concat();
    let irqs = [irq(100, false), irq(101, true), irq(110, false), irq(111, false), irq(112, true)];
    let data = [fixed, refs, irqs.concat(), irq(120, true)].concat();
    node(3, 3, 1, &data, &[mapping((0, 0xFFFF, 0x1_0000, ITS, 0))])
}

pub fn rmr(ranges: &[(u64, u64)]) -> Vec<u8> {
    let head = [u32le(0), u32le(ranges.len() as u32), u32le(28)].concat();
    let one = |&(b, l): &(u64, u64)| [u64le(b), u64le(l), u32le(0)].concat();
    let body: Vec<u8> = ranges.iter().flat_map(one).collect();
    let maps = [
        mapping((0, 1, 0x80, SMMU, 0)),
        mapping((0, 0, 0x90, SMMU, 1)),
        mapping((0, 0, 7, ITS, 1)),
    ];
    node(6, 3, 4, &[head, body].concat(), &maps)
}

pub fn nodes_v2(context_at: u32, ranges: &[(u64, u64)]) -> Vec<Vec<u8>> {
    let its = node(0, 1, 0, &[u32le(1), u32le(7)].concat(), &[]);
    let rc_data = [u32le(1), vec![0; 4], u32le(1), u32le(0), vec![48, 0, 0, 0]].concat();
    let rc = node(2, 1, 2, &rc_data, &[mapping((0, 0xFF, 0x100, SMMU, 0))]);
    vec![its, smmu_v2(context_at), rc, rmr(ranges)]
}

pub const RANGES: [(u64, u64); 2] = [(0x8000_0000, 0x10_0000), (0x9000_0000, 0x1000)];
