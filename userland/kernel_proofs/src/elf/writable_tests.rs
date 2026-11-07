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

//! The relocation loop's writable-range check, collected once per load
//! (`elf/loader/core/writable.rs`), answers as the per-entry scan it
//! replaced did: for a write of `len` bytes at `offset`, is there a PT_LOAD
//! that is writable, whose end does not overflow, holding the whole write.

#[path = "../../../../src/elf/loader/core/writable.rs"]
mod writable;

use writable::{collect, covers};

/// A program header as the per-entry scan read it.
#[derive(Clone, Copy)]
struct Ph {
    load: bool,
    writable: bool,
    vaddr: u64,
    memsz: u64,
}

/// The per-entry scan of (`reloc_in_writable_load`), over headers.
fn scanned(phs: &[Ph], r_offset: u64, len: u64) -> bool {
    let Some(end) = r_offset.checked_add(len) else {
        return false;
    };
    for ph in phs {
        if !ph.load || !ph.writable {
            continue;
        }
        let Some(seg_end) = ph.vaddr.checked_add(ph.memsz) else {
            continue;
        };
        if r_offset >= ph.vaddr && end <= seg_end {
            return true;
        }
    }
    false
}

fn collected(phs: &[Ph]) -> Vec<writable::Range> {
    collect(phs.iter().filter(|p| p.load && p.writable).map(|p| (p.vaddr, p.memsz)))
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn near(&mut self, anchors: &[u64]) -> u64 {
        let a = anchors[(self.next() % anchors.len() as u64) as usize];
        a.wrapping_add(self.next() % 24).wrapping_sub(12)
    }
}

#[test]
fn collected_ranges_answer_as_the_per_entry_scan() {
    let mut rng = Rng(0x5eed_1234_abcd_0001);
    for _ in 0..20_000 {
        let n = (rng.next() % 6) as usize;
        let phs: Vec<Ph> = (0..n)
            .map(|_| {
                let r = rng.next();
                let vaddr = match r % 4 {
                    0 => u64::MAX - (rng.next() % 64),
                    1 => 0,
                    _ => (rng.next() % 0x10_0000) & !0xfff,
                };
                Ph { load: r & 16 != 0, writable: r & 32 != 0, vaddr, memsz: rng.next() % 0x4000 }
            })
            .collect();
        let ranges = collected(&phs);
        let mut anchors: Vec<u64> = vec![0, u64::MAX, u64::MAX - 7];
        for p in &phs {
            anchors.push(p.vaddr);
            anchors.push(p.vaddr.wrapping_add(p.memsz));
        }
        for _ in 0..64 {
            let off = rng.near(&anchors);
            for len in [8u64, 1, 0] {
                assert_eq!(covers(&ranges, off, len), scanned(&phs, off, len), "offset {off:#x} len {len}");
            }
        }
    }
}

#[test]
fn a_write_that_straddles_a_segment_end_is_refused() {
    let phs = [Ph { load: true, writable: true, vaddr: 0x1000, memsz: 0x10 }];
    let r = collected(&phs);
    assert!(covers(&r, 0x1008, 8));
    assert!(!covers(&r, 0x1009, 8));
    assert!(!covers(&r, 0xfff, 8));
}

#[test]
fn read_only_segments_and_overflowing_ends_hold_nothing() {
    let phs = [
        Ph { load: true, writable: false, vaddr: 0x1000, memsz: 0x1000 },
        Ph { load: true, writable: true, vaddr: u64::MAX - 4, memsz: 16 },
    ];
    let r = collected(&phs);
    assert!(!covers(&r, 0x1000, 8));
    assert!(!covers(&r, u64::MAX - 4, 1));
    assert!(!covers(&r, u64::MAX - 1, 8));
}
