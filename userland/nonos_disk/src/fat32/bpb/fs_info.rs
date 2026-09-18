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

//! FSInfo at sector 1: the free-cluster count and the first free cluster.
//! Both are exact here because the volume is written once from a known
//! tree; a driver that trusts them gets the truth.

use alloc::vec::Vec;

use super::super::geometry::Geometry;
use crate::sink::SECTOR_SIZE;

pub fn fs_info(geo: &Geometry, used_clusters: u64) -> Vec<u8> {
    let mut s = alloc::vec![0u8; SECTOR_SIZE];
    s[0..4].copy_from_slice(&0x4161_5252u32.to_le_bytes());
    s[484..488].copy_from_slice(&0x6141_7272u32.to_le_bytes());
    let free = geo.data_clusters.saturating_sub(used_clusters) as u32;
    s[488..492].copy_from_slice(&free.to_le_bytes());
    s[492..496].copy_from_slice(&(2 + used_clusters as u32).to_le_bytes());
    s[508..512].copy_from_slice(&0xAA55_0000u32.to_le_bytes());
    s
}
