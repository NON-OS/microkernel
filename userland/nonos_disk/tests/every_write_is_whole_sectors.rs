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

//! A block driver takes whole sectors and nothing else. The sink here
//! refuses what a driver refuses and stores nothing, so the image can be the
//! size of the real one: a loader that is an exact multiple of the sector
//! and a kernel that is not, on an eight gigabyte disk. The order is checked
//! too: the old tables are wiped first, the store's header after its body,
//! the plan after the store, the table after both with the primary GPT
//! header last, and the flush after it.

#[path = "common/entropy.rs"]
mod entropy;
#[path = "common/strict_sink.rs"]
mod strict_sink;

use nonos_disk::{install, NonosImage, StoreImage, SECTOR_SIZE};
use strict_sink::StrictSink;

#[test]
fn every_write_is_whole_sectors() {
    /*
     * The loader fills its clusters exactly (a multiple of 4096), the kernel
     * does not: both shapes have been shipped, and the first one once queued
     * an empty tail write that the driver refused.
     */
    let (boot_efi, kernel_bin) = (vec![0xAAu8; 14_225_408], vec![0x55u8; 89_971_099]);
    let image = NonosImage { boot_efi: &boot_efi, kernel_bin: &kernel_bin, boot_cfg: b"x=1\n" };
    let sectors = (8u64 << 30) / SECTOR_SIZE as u64;
    let mut sink = StrictSink { sectors, writes: Vec::new(), flushed_after: None };
    let result = install(&mut sink, &image, StoreImage::empty(), entropy::ENTROPY, &mut |_| {});
    let bad: Vec<_> = sink.writes.iter().filter(|(_, n)| *n == 0 || n % SECTOR_SIZE != 0).collect();
    assert!(bad.is_empty(), "writes a driver refuses: {bad:?}");
    /*
     * The read-back sees zeros, so the install reports a mismatch; what
     * matters here is that the write phase went to the end, in order.
     */
    let written: usize = sink.writes.iter().map(|(_, n)| n).sum();
    assert!(written > boot_efi.len() + kernel_bin.len(), "stopped early: {result:?}");
    assert_eq!(sink.writes[..4], [(0, 1024), (sectors - 1, 512), (256, 512), (65536, 512)]);
    let last = |w: (u64, usize)| sink.writes.iter().rposition(|x| *x == w).unwrap();
    let body = sink.writes.iter().rposition(|x| x.0 == 257).unwrap();
    assert!(body < last((256, 512)) && last((256, 512)) < last((65536, 512)), "body, header, plan");
    assert_eq!(sink.writes[last((65536, 512)) + 1], (sectors - 33, 16384), "then the backup table");
    assert_eq!(sink.writes.last(), Some(&(1, 512)), "the primary GPT header is last");
    assert_eq!(sink.flushed_after, Some(sink.writes.len()), "and the flush follows it");
}
