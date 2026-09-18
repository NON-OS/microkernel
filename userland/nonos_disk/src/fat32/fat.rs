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

//! The allocation table. Every run is contiguous, so each chain is a
//! count-up ending in the end-of-chain mark; every other entry is zero,
//! which is "free". The first two entries are the media mark and the
//! clean-shutdown flags the specification reserves.

use alloc::vec::Vec;

use super::geometry::Geometry;
use super::tree::Run;
use crate::sink::SECTOR_SIZE;

const END_OF_CHAIN: u32 = 0x0FFF_FFFF;
const MEDIA_ENTRY: u32 = 0x0FFF_FFF8;

pub fn build(geo: &Geometry, runs: &[Run<'_>]) -> Vec<u8> {
    let mut fat = alloc::vec![0u8; geo.fat_sectors as usize * SECTOR_SIZE];
    set(&mut fat, 0, MEDIA_ENTRY);
    set(&mut fat, 1, END_OF_CHAIN);
    for run in runs {
        if run.clusters == 0 {
            continue;
        }
        let last = run.first_cluster + run.clusters - 1;
        for c in run.first_cluster..last {
            set(&mut fat, c, c + 1);
        }
        set(&mut fat, last, END_OF_CHAIN);
    }
    fat
}

fn set(fat: &mut [u8], cluster: u32, value: u32) {
    let o = cluster as usize * 4;
    fat[o..o + 4].copy_from_slice(&(value & 0x0FFF_FFFF).to_le_bytes());
}
