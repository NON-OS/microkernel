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

//! Native C-state structures as firmware lays them out (recalled from
//! Intel's LPIT description, unverified).

pub fn gas(space: u8, width: u8, address: u64) -> Vec<u8> {
    [vec![space, width, 0, 4], address.to_le_bytes().to_vec()].concat()
}

pub fn native(
    id: u16,
    flags: u32,
    residency: u32,
    latency: u32,
    counter: &[u8],
    freq: u64,
) -> Vec<u8> {
    let head = [0u32.to_le_bytes(), 56u32.to_le_bytes()].concat();
    let ids = [id.to_le_bytes().to_vec(), vec![0, 0], flags.to_le_bytes().to_vec()].concat();
    let times = [gas(0x7F, 0, 0), residency.to_le_bytes().to_vec(), latency.to_le_bytes().to_vec()];
    [head, ids, times.concat(), counter.to_vec(), freq.to_le_bytes().to_vec()].concat()
}

pub fn lpit(structures: &[Vec<u8>]) -> Vec<u8> {
    [b"LPIT".to_vec(), vec![0; 32], structures.concat()].concat()
}
