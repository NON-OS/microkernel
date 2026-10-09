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

//! The installer's block client speaks 512-byte sectors to drivers that
//! speak their disk's own blocks (`nonos_blk_client/src/device/span.rs`).
//! An NVMe namespace formatted with 4096-byte blocks took sector addresses
//! as block addresses, so every write landed eight times too far in and
//! the disk showed an eighth of its size. Driven here against a disk in
//! memory that checks every request the way the NVMe driver does: a whole
//! number of its blocks, within its transfer limit, inside the disk. Any
//! mix of sector reads and writes must read back as a plain 512-byte disk
//! would, and a write must never change a sector it was not given.

#[path = "../../nonos_blk_client/src/device/span.rs"]
mod span;

use span::{max_transfer_bytes, read, write, Geometry, Native, DATA_BYTES, SECTOR};

struct Disk {
    g: Geometry,
    bytes: Vec<u8>,
    reads: usize,
    writes: usize,
    /// A block a request touching fails, as a driver's E_IO.
    bad: Option<u64>,
}

impl Disk {
    fn new(g: Geometry, blocks: u64) -> Disk {
        let n = blocks as usize * g.lba_size() as usize;
        let bytes = (0..n).map(|i| (i * 7 + i / 512) as u8).collect();
        Disk { g, bytes, reads: 0, writes: 0, bad: None }
    }

    fn check(&self, lba: u64, len: usize) -> Result<std::ops::Range<usize>, i32> {
        let block = self.g.lba_size() as usize;
        assert!(len > 0 && len.is_multiple_of(block), "request of {len} bytes at {block}-byte blocks");
        assert!(len <= self.g.max_bytes() as usize, "request of {len} bytes past the limit");
        let start = lba as usize * block;
        assert!(start + len <= self.bytes.len(), "request past the end of the disk");
        let count = (len / block) as u64;
        if self.bad.is_some_and(|b| (lba..lba + count).contains(&b)) {
            return Err(-5);
        }
        Ok(start..start + len)
    }
}

impl Native for Disk {
    type Error = i32;

    fn read_native(&mut self, lba: u64, out: &mut [u8]) -> Result<(), i32> {
        self.reads += 1;
        let r = self.check(lba, out.len())?;
        out.copy_from_slice(&self.bytes[r]);
        Ok(())
    }

    fn write_native(&mut self, lba: u64, data: &[u8]) -> Result<(), i32> {
        self.writes += 1;
        let r = self.check(lba, data.len())?;
        self.bytes[r].copy_from_slice(data);
        Ok(())
    }
}

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn the_geometry_follows_the_nvme_driver_rule() {
    assert_eq!(max_transfer_bytes(0), DATA_BYTES);
    assert_eq!(max_transfer_bytes(1), 8192);
    assert_eq!(max_transfer_bytes(2), 16384);
    assert_eq!(max_transfer_bytes(3), DATA_BYTES);
    assert_eq!(max_transfer_bytes(200), DATA_BYTES);

    let g = Geometry::nvme(4096, 0).unwrap();
    assert_eq!((g.lba_size(), g.max_bytes(), g.per_block()), (4096, DATA_BYTES, 8));
    let g = Geometry::nvme(4096, 1).unwrap();
    assert_eq!(g.max_bytes(), 8192);
    let g = Geometry::nvme(512, 0).unwrap();
    assert_eq!(g, Geometry::SECTORS);
    for bad in [0, 256, 520, 3000, 65536] {
        assert_eq!(Geometry::nvme(bad, 0), None, "{bad}");
    }
}

#[test]
fn a_4096_byte_namespace_is_sized_in_sectors_from_its_blocks() {
    let g = Geometry::nvme(4096, 0).unwrap();
    /* A 512 GB part formatted 4Kn: 125_026_902 blocks. */
    let sectors = g.sectors(125_026_902);
    assert_eq!(sectors, 1_000_215_216);
    assert_eq!(sectors * SECTOR as u64, 512_110_190_592);
    assert_eq!(g.sectors(u64::MAX), u64::MAX, "a capacity that would wrap saturates");
    assert_eq!(Geometry::SECTORS.sectors(1_000_215_216), 1_000_215_216);
}

#[test]
fn any_sector_reads_and_writes_read_back_as_a_512_byte_disk() {
    for (g, blocks) in [
        (Geometry::SECTORS, 600u64),
        (Geometry::nvme(4096, 0).unwrap(), 80),
        (Geometry::nvme(4096, 1).unwrap(), 80),
        (Geometry::nvme(1024, 0).unwrap(), 300),
    ] {
        let mut disk = Disk::new(g, blocks);
        let mut model = disk.bytes.clone();
        let sectors = model.len() / SECTOR;
        let mut s = 0x2545_F491_4F6C_DD1Du64 ^ u64::from(g.lba_size());
        for round in 0..3000 {
            let lba = next(&mut s) % sectors as u64;
            let room = sectors - lba as usize;
            let n = 1 + (next(&mut s) as usize % room.min(150));
            let range = lba as usize * SECTOR..(lba as usize + n) * SECTOR;
            if round % 2 == 0 {
                let data: Vec<u8> = (0..n * SECTOR).map(|_| next(&mut s) as u8).collect();
                write(&mut disk, &g, lba, &data).unwrap();
                model[range].copy_from_slice(&data);
                assert!(disk.bytes == model, "write of {n} at {lba}, {}-byte blocks", g.lba_size());
            } else {
                let mut out = vec![0u8; n * SECTOR];
                read(&mut disk, &g, lba, &mut out).unwrap();
                assert!(out == model[range], "read of {n} at {lba}, {}-byte blocks", g.lba_size());
            }
        }
    }
}

#[test]
fn an_aligned_write_reads_nothing_and_a_ragged_one_reads_only_its_edges() {
    let g = Geometry::nvme(4096, 0).unwrap();
    let mut disk = Disk::new(g, 64);
    write(&mut disk, &g, 8, &vec![1u8; 16 * 4096]).unwrap();
    assert_eq!((disk.reads, disk.writes), (0, 2), "sixteen blocks in two requests of eight");

    let mut disk = Disk::new(g, 64);
    write(&mut disk, &g, 1, &vec![2u8; 512]).unwrap();
    assert_eq!((disk.reads, disk.writes), (1, 1), "one sector inside one block");

    let mut disk = Disk::new(g, 64);
    write(&mut disk, &g, 7, &vec![3u8; 10 * 512]).unwrap();
    assert_eq!((disk.reads, disk.writes), (2, 1), "both edge blocks, once each");

    let mut disk = Disk::new(g, 64);
    write(&mut disk, &g, 0, &vec![4u8; 9 * 512]).unwrap();
    assert_eq!(disk.reads, 1, "only the tail block");
}

#[test]
fn a_refused_request_stops_the_transfer_and_is_returned() {
    let g = Geometry::nvme(4096, 0).unwrap();
    let mut disk = Disk::new(g, 64);
    disk.bad = Some(20);
    let before = disk.bytes.clone();
    assert_eq!(write(&mut disk, &g, 8 * 8, &vec![9u8; 32 * 4096]), Err(-5));
    let block = 4096;
    assert_eq!(disk.bytes[..8 * block], before[..8 * block], "nothing before the range");
    assert!(disk.bytes[8 * block..16 * block].iter().all(|&b| b == 9), "the request before it");
    assert_eq!(
        disk.bytes[16 * block..],
        before[16 * block..],
        "nothing from the failed request on"
    );

    let mut out = vec![0u8; 3 * 512];
    disk.bad = Some(0);
    assert_eq!(read(&mut disk, &g, 1, &mut out), Err(-5));
}
