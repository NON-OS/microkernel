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

//! Sector spans on a disk of 4096-byte blocks, at its edges: the client
//! turns 512-byte sectors into whole blocks, splits them at what one
//! request moves, and reads a block the range covers only in part before
//! writing it back. The disk checks each request as the NVMe driver does.

use super::disk::{g, Disk, BLOCK};
use crate::span::{read, write};

#[test]
fn the_last_sector_of_a_4kn_disk_is_written_alone() {
    let mut d = Disk::new(16);
    let before = d.bytes.clone();
    write(&mut d, &g(), 16 * 8 - 1, &[0xEE; 512]).expect("the last sector");
    assert_eq!(d.sent, [(false, 15, 1), (true, 15, 1)], "read the block, write it back");
    assert_eq!(&d.bytes[..16 * BLOCK - 512], &before[..16 * BLOCK - 512]);
    assert!(d.bytes[16 * BLOCK - 512..].iter().all(|&b| b == 0xEE));
}

#[test]
fn a_ragged_span_past_one_request_is_split_and_read_back_whole() {
    let mut d = Disk::new(16);
    let mut model = d.bytes.clone();
    let data: Vec<u8> = (0..70 * 512).map(|i| (i % 251) as u8).collect();
    write(&mut d, &g(), 3, &data).expect("70 sectors from sector 3");
    model[3 * 512..73 * 512].copy_from_slice(&data);
    assert_eq!(d.bytes, model);
    let writes: Vec<_> = d.sent.iter().filter(|s| s.0).map(|s| (s.1, s.2)).collect();
    assert_eq!(writes, [(0, 8), (8, 2)], "ten blocks in requests of at most eight");
    let mut out = vec![0u8; 70 * 512];
    read(&mut d, &g(), 3, &mut out).expect("read back");
    assert_eq!(out, data);
}

#[test]
fn a_refused_edge_read_writes_nothing() {
    let mut d = Disk::new(16);
    d.bad = Some(0);
    let before = d.bytes.clone();
    assert_eq!(write(&mut d, &g(), 1, &[0xEE; 8 * 512]), Err(-5));
    assert_eq!(d.bytes, before, "a block was written over one that never read");
}
