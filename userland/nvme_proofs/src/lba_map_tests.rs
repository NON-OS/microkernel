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

//! The kernel client counts 512-byte sectors; the capsule counts the
//! namespace's own LBAs. The proofs run the client's real map: every plan
//! covers exactly the bytes asked for, each unaligned end lies inside one
//! LBA, the whole part is LBA aligned and cut into commands the capsule
//! takes, and a disk driven through the plan (reads of whole LBAs, writes by
//! read-modify-write at the ends) reads and writes exactly what a 512-byte
//! disk would. The client's per-command ceiling is the capsule's own.

use crate::admin::{ControllerIdentity, NamespaceIdentity};
use crate::lba_map::{
    addressable, capacity_sectors, chunks, lbas_per_request, max_transfer_bytes, plan, Plan, SECTOR,
};
use crate::nvm::{max_transfer_bytes as capsule_max_transfer_bytes, NamespaceGeometry};

/// The capsule's data buffer, the most one command moves.
const BUFFER: u64 = 64 * 512;

/// Every byte of `[start, end)` named once, in order: (disk byte, buffer
/// byte) pairs walked from the plan's parts.
fn covered(p: &Plan, lba_size: u64) -> Vec<(u64, usize)> {
    let mut out = Vec::new();
    if let Some(h) = p.head {
        assert!(h.offset + h.len <= lba_size as usize, "a head runs past its LBA");
        for i in 0..h.len {
            out.push((h.lba * lba_size + (h.offset + i) as u64, h.at + i));
        }
    }
    if let Some(w) = p.body {
        assert!(w.lbas > 0);
        for i in 0..(w.lbas * lba_size) {
            out.push((w.lba * lba_size + i, w.at + i as usize));
        }
    }
    if let Some(t) = p.tail {
        assert_eq!(t.offset, 0, "a tail starts its LBA");
        assert!(t.len < lba_size as usize, "a whole LBA is never a tail");
        for i in 0..t.len {
            out.push((t.lba * lba_size + i as u64, t.at + i));
        }
    }
    out
}

#[test]
fn every_plan_covers_exactly_the_sectors_asked_for() {
    for lba_size in [512u32, 1024, 4096, 8192] {
        let ls = lba_size as u64;
        for sector in 0..40u64 {
            for sectors in 1..40usize {
                let bytes = sectors * SECTOR;
                let p = plan(sector, bytes, lba_size).expect("an addressable plan");
                let cov = covered(&p, ls);
                assert_eq!(cov.len(), bytes, "{lba_size}: {sector}+{sectors}");
                for (k, &(disk, buf)) in cov.iter().enumerate() {
                    assert_eq!(disk, sector * 512 + k as u64);
                    assert_eq!(buf, k);
                }
                if let Some(w) = p.body {
                    assert_eq!((w.at as u64 + sector * 512) % ls, 0, "the body is LBA aligned");
                }
                if lba_size == 512 {
                    assert!(p.head.is_none() && p.tail.is_none(), "512-byte LBAs need no RMW");
                    assert_eq!(p.body.unwrap().lba, sector);
                    assert_eq!(p.body.unwrap().lbas, sectors as u64);
                }
                let first = sector * 512;
                let end = first + bytes as u64;
                let aligned = first.is_multiple_of(ls) && end.is_multiple_of(ls);
                assert_eq!(p.head.is_none() && p.tail.is_none(), aligned, "RMW only off alignment");
            }
        }
    }
}

#[test]
fn the_named_4k_spans() {
    // (first sector, sectors) -> (head lba, offset, len), (body lba, lbas), (tail lba, len)
    let p = plan(0, 4096, 4096).unwrap();
    assert_eq!((p.head, p.tail), (None, None));
    assert_eq!((p.body.unwrap().lba, p.body.unwrap().lbas), (0, 1));
    // Sector 1 alone: inside LBA 0, bytes 512..1024, read and patched whole.
    let p = plan(1, 512, 4096).unwrap();
    let h = p.head.unwrap();
    assert_eq!((h.lba, h.offset, h.len, h.at), (0, 512, 512, 0));
    assert_eq!((p.body, p.tail), (None, None));
    // Sectors 7..17: the last sector of LBA 0, LBA 1 whole, 1 sector of LBA 2.
    let p = plan(7, 10 * 512, 4096).unwrap();
    let h = p.head.unwrap();
    assert_eq!((h.lba, h.offset, h.len, h.at), (0, 3584, 512, 0));
    let w = p.body.unwrap();
    assert_eq!((w.lba, w.lbas, w.at), (1, 1, 512));
    let t = p.tail.unwrap();
    assert_eq!((t.lba, t.offset, t.len, t.at), (2, 0, 512, 4608));
    // Sector 8 for one sector: the first sector of LBA 1, a tail alone.
    let p = plan(8, 512, 4096).unwrap();
    assert_eq!((p.head, p.body), (None, None));
    let t = p.tail.unwrap();
    assert_eq!((t.lba, t.len, t.at), (1, 512, 0));
}

#[test]
fn a_plan_never_overflows_and_refuses_lba_sizes_it_cannot_map() {
    assert!(plan(u64::MAX, 512, 4096).is_none());
    assert!(plan(u64::MAX / 512, 1024, 4096).is_none());
    assert!(plan(u64::MAX / 512, 0, 4096).is_some());
    for bad in [0u32, 256, 511, 520, 3072, 4097] {
        assert!(plan(0, 512, bad).is_none(), "{bad}");
        assert!(!addressable(bad, BUFFER as u32), "{bad}");
    }
    for good in [512u32, 1024, 2048, 4096, 8192, 16384, 32768] {
        assert!(addressable(good, BUFFER as u32), "{good}");
    }
    assert!(!addressable(65536, BUFFER as u32), "one LBA past the buffer");
}

#[test]
fn chunks_cover_the_body_and_never_exceed_the_ceiling() {
    for lba_size in [512u32, 4096] {
        for per in 1..=70u64 {
            for lbas in 1..=150u64 {
                let w = crate::lba_map::Whole { lba: 1000, lbas, at: 7 };
                let mut next_lba = 1000;
                let mut next_off = 0usize;
                for (lba, n, off) in chunks(w, per, lba_size) {
                    assert!(n >= 1 && n <= per);
                    assert_eq!(lba, next_lba);
                    assert_eq!(off, next_off);
                    next_lba += n;
                    next_off += n as usize * lba_size as usize;
                }
                assert_eq!(next_lba, 1000 + lbas);
            }
        }
    }
}

#[test]
fn the_client_ceiling_is_the_capsules() {
    for mdts in 0..=255u8 {
        assert_eq!(max_transfer_bytes(mdts, BUFFER), capsule_max_transfer_bytes(mdts), "{mdts}");
        for lbads in [9u8, 12] {
            let mut ns = [0u8; 4096];
            ns[0..8].copy_from_slice(&(1u64 << 30).to_le_bytes());
            ns[0x82] = lbads;
            let mut ctrl = [0u8; 4096];
            ctrl[0x4d] = mdts;
            let g = NamespaceGeometry::check(
                &ControllerIdentity::parse(&ctrl),
                &NamespaceIdentity::parse(1, &ns),
            )
            .expect("a plain namespace is served");
            let per = lbas_per_request(mdts, 1 << lbads, BUFFER);
            assert_eq!(per, g.max_sectors as u64, "MDTS {mdts}, LBADS {lbads}");
            assert!(per >= 1);
        }
    }
}

#[test]
fn capacity_is_reported_in_512_byte_sectors() {
    assert_eq!(capacity_sectors(1000, 512), Some(1000));
    assert_eq!(capacity_sectors(1000, 4096), Some(8000));
    // A 1 TB drive at 4 KiB LBAs.
    assert_eq!(capacity_sectors(244_190_646, 4096), Some(1_953_525_168));
    assert_eq!(capacity_sectors(u64::MAX / 4, 4096), None);
}

/// A namespace of `lba_size`-byte LBAs that only moves whole LBAs, and the
/// client's read and write on it, built from the plan exactly as
/// client/read_blocks.rs and client/write_blocks.rs build theirs.
struct Disk {
    lba_size: usize,
    bytes: Vec<u8>,
    per: u64,
    commands: usize,
}

impl Disk {
    fn read_lbas(&mut self, lba: u64, n: u64, out: &mut [u8]) {
        assert!(n >= 1 && n <= self.per, "a command past the capsule's ceiling");
        assert_eq!(out.len(), n as usize * self.lba_size);
        let at = lba as usize * self.lba_size;
        out.copy_from_slice(&self.bytes[at..at + out.len()]);
        self.commands += 1;
    }

    fn write_lbas(&mut self, lba: u64, n: u64, data: &[u8]) {
        assert!(n >= 1 && n <= self.per, "a command past the capsule's ceiling");
        assert_eq!(data.len(), n as usize * self.lba_size);
        let at = lba as usize * self.lba_size;
        self.bytes[at..at + data.len()].copy_from_slice(data);
        self.commands += 1;
    }

    fn read(&mut self, sector: u64, out: &mut [u8]) {
        let ls = self.lba_size;
        let p = plan(sector, out.len(), ls as u32).unwrap();
        for part in [p.head, p.tail].into_iter().flatten() {
            let mut block = vec![0u8; ls];
            self.read_lbas(part.lba, 1, &mut block);
            out[part.at..part.at + part.len]
                .copy_from_slice(&block[part.offset..part.offset + part.len]);
        }
        if let Some(w) = p.body {
            for (lba, n, off) in chunks(w, self.per, ls as u32) {
                let at = w.at + off;
                self.read_lbas(lba, n, &mut out[at..at + n as usize * ls]);
            }
        }
    }

    fn write(&mut self, sector: u64, data: &[u8]) {
        let ls = self.lba_size;
        let p = plan(sector, data.len(), ls as u32).unwrap();
        for part in [p.head, p.tail].into_iter().flatten() {
            let mut block = vec![0u8; ls];
            self.read_lbas(part.lba, 1, &mut block);
            block[part.offset..part.offset + part.len]
                .copy_from_slice(&data[part.at..part.at + part.len]);
            self.write_lbas(part.lba, 1, &block);
        }
        if let Some(w) = p.body {
            for (lba, n, off) in chunks(w, self.per, ls as u32) {
                let at = w.at + off;
                self.write_lbas(lba, n, &data[at..at + n as usize * ls]);
            }
        }
    }
}

#[test]
fn a_4k_disk_through_the_client_reads_and_writes_as_a_512_byte_disk() {
    let mut s = 0x9e37_79b9_7f4a_7c15u64;
    let mut rng = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    for (lba_size, mdts) in [(4096usize, 0u8), (4096, 1), (512, 1), (512, 0), (8192, 0)] {
        let per = lbas_per_request(mdts, lba_size as u32, BUFFER);
        let size = 64 * 4096;
        let init: Vec<u8> = (0..size).map(|_| rng() as u8).collect();
        let mut disk = Disk { lba_size, bytes: init.clone(), per, commands: 0 };
        let mut model = init;
        for _ in 0..2_000 {
            let sectors = 1 + (rng() % 100) as usize;
            let sector = rng() % (size / 512 - sectors) as u64;
            let at = sector as usize * 512;
            let len = sectors * 512;
            if rng() % 2 == 0 {
                let data: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                disk.write(sector, &data);
                model[at..at + len].copy_from_slice(&data);
                assert_eq!(disk.bytes, model, "a write touched bytes it was not given");
            } else {
                let mut out = vec![0u8; len];
                disk.read(sector, &mut out);
                assert_eq!(out, model[at..at + len], "read {sector}+{sectors}");
            }
        }
        assert!(disk.commands > 0);
    }
}
