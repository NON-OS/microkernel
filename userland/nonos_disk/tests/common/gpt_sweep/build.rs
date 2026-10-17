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

//! A sound GPT disk: a protective MBR, a header naming 1 to 1024 entries
//! of 128 x 2^n bytes in at most 128 KiB, and partitions that never share
//! a sector, on disks of 2^12 to 2^40 sectors spread over powers of two.

use super::disk::{Disk, ARRAY_ROOM};
use super::mbr::mbr;
use super::parts::{kind, spans};
use super::rng::Rng;

pub fn disk(r: &mut Rng) -> Disk {
    let size = 128u64 << r.range(0, 3);
    let most = 128 * 1024 / size;
    let k = r.range(0, most.ilog2() as u64);
    let n = r.range(1 << k, (2 << k) - 1).min(most);
    let lba = r.range(2, 18);
    let first = lba + (n * size).div_ceil(512) + r.range(0, 2048);
    let sectors = 1u64 << r.range(first.ilog2() as u64 + 4, 40);
    let last = sectors - 1 - r.range(1, 34);
    let mut d =
        Disk { sectors, mbr: mbr(r, sectors), header: vec![0; 512], array: vec![0; ARRAY_ROOM] };
    let used = ((n * size).div_ceil(512) * 512) as usize;
    d.array[..used].copy_from_slice(&r.bytes(used));
    let hsize = if r.one_in(4) { r.range(92, 512) } else { 92 };
    d.header[..8].copy_from_slice(b"EFI PART");
    d.header[92..].copy_from_slice(&r.bytes(420));
    for (at, v, w) in
        [(8, 0x1_0000, 4), (12, hsize, 4), (20, 0, 4), (24, 1, 8), (32, sectors - 1, 8)]
    {
        d.put(at, v, w);
    }
    for (at, v, w) in [(40, first, 8), (48, last, 8), (72, lba, 8), (80, n, 4), (84, size, 4)] {
        d.put(at, v, w);
    }
    d.header[56..72].copy_from_slice(&r.bytes(16));
    let parts = spans(r, first, last, n.min(8));
    let mut slots: Vec<usize> = (0..n as usize).collect();
    for i in 0..n as usize {
        // An unused entry may still name sectors inside the usable range.
        let (a, b) = (r.range(first, last), r.range(first, last));
        let e = d.entry(i);
        e[..16].fill(0);
        e[32..48].copy_from_slice(&[a.min(b).to_le_bytes(), a.max(b).to_le_bytes()].concat());
    }
    for (a, b) in parts {
        let slot = slots.swap_remove(r.range(0, slots.len() as u64 - 1) as usize);
        let e = d.entry(slot);
        e[..16].copy_from_slice(&kind(r));
        e[32..40].copy_from_slice(&a.to_le_bytes());
        e[40..48].copy_from_slice(&b.to_le_bytes());
    }
    d.seal();
    d
}
