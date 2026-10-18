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

//! A sound GPT disk: a protective MBR, a primary header naming 1 to 1024
//! entries of 128 x 2^n bytes in at most 128 KiB, partitions that never
//! share a sector, and the backup pair at the end naming the same, on
//! disks of 2^5 to 2^40 sectors spread over powers of two.

use super::disk::{Disk, ARRAY_ROOM};
use super::mbr::mbr;
use super::parts::place;
use super::rng::Rng;

pub fn disk(r: &mut Rng) -> Disk {
    let size = 128u64 << r.range(0, 3);
    let most = 128 * 1024 / size;
    let k = r.range(0, most.ilog2() as u64);
    let n = r.range(1 << k, (2 << k) - 1).min(most);
    let lba = r.range(2, 18);
    let first = lba + (n * size).div_ceil(512) + r.range(0, 2048);
    let sectors = 1u64 << r.range(first.ilog2() as u64 + 4, 40);
    let s = (n * size).div_ceil(512);
    let last = sectors - 2 - s - r.range(0, 32);
    let (heads, arrays) = ([vec![0; 512], vec![0; 512]], [vec![0; ARRAY_ROOM], vec![]]);
    let array_lba = [lba, last + 1];
    let mut d = Disk { sectors, mbr: mbr(r, sectors), heads, arrays, array_lba };
    let used = (s * 512) as usize;
    d.arrays[0][..used].copy_from_slice(&r.bytes(used));
    let hsize = if r.one_in(4) { r.range(92, 512) } else { 92 };
    d.heads[0][..8].copy_from_slice(b"EFI PART");
    d.heads[0][92..].copy_from_slice(&r.bytes(420));
    for (at, v, w) in
        [(8, 0x1_0000, 4), (12, hsize, 4), (20, 0, 4), (24, 1, 8), (32, sectors - 1, 8)]
    {
        d.put(0, at, v, w);
    }
    for (at, v, w) in [(40, first, 8), (48, last, 8), (72, lba, 8), (80, n, 4), (84, size, 4)] {
        d.put(0, at, v, w);
    }
    d.heads[0][56..72].copy_from_slice(&r.bytes(16));
    place(r, &mut d, first, last, n);
    d.seal(0);
    backup(&mut d, last);
    d
}

/// The backup pair as the primary's, at the end: its own LBA the last, the
/// primary's 1, its array after LastUsableLBA.
fn backup(d: &mut Disk, last: u64) {
    (d.heads[1], d.arrays[1]) = (d.heads[0].clone(), d.arrays[0].clone());
    debug_assert!(d.arrays[1].len() == super::disk::ARRAY_ROOM);
    for (at, v) in [(24, d.sectors - 1), (32, 1), (72, last + 1)] {
        d.put(1, at, v, 8);
    }
    d.seal(1);
}
