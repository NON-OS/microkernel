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

//! Hostile IDENTIFY blocks: whatever 512 bytes a drive answers, the driver
//! either refuses the disk or serves a count that obeys every rule, and no
//! word outside the ones the rule names moves the answer.

use crate::identity::{capacity, Refusal};
use crate::identity_tests::{set_lba48, set_long_sector, Block, LBA48_LIMIT};

/// The words the rule reads. Every other word of the block is the drive's
/// to fill with anything.
const NAMED: [usize; 10] = [83, 86, 87, 100, 101, 102, 103, 106, 117, 118];

// Miri runs the same harnesses with fewer rounds; the full counts are for
// the native fuzz.
fn rounds(full: u64) -> u64 {
    if cfg!(miri) {
        300
    } else {
        full
    }
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn seeded(seed: u64) -> u64 {
    seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1
}

/// A block of random words, with the words the rule reads pushed toward the
/// shapes that pass, so every branch runs on most seed ranges: each validity
/// field reads 01b and each feature bit is set three times in four, word 106
/// often names a sector of 512 bytes or of some other length, and the 48-bit
/// count often sits on or next to an edge.
fn hostile_block(s: &mut u64) -> Block {
    let mut w: Block = [0; 256];
    for x in w.iter_mut() {
        *x = xorshift(s) as u16;
    }
    let r = xorshift(s);
    let often = |k: u32| (r >> k) & 3 != 0;
    if often(0) {
        w[83] = (w[83] & !0xc000) | 0x4000;
    }
    if often(2) {
        w[87] = (w[87] & !0xc000) | 0x4000;
    }
    if often(4) {
        w[83] |= 1 << 10;
    }
    if often(6) {
        w[86] |= 1 << 10;
    }
    match (r >> 8) % 4 {
        0 => w[106] = 0x6000,
        1 => set_long_sector(&mut w, 256),
        2 => set_long_sector(&mut w, (xorshift(s) % 4096) as u32),
        _ => {}
    }
    match (r >> 16) % 6 {
        0 => set_lba48(&mut w, 0),
        1 => set_lba48(&mut w, xorshift(s) % LBA48_LIMIT),
        2 => set_lba48(&mut w, LBA48_LIMIT - 1 - xorshift(s) % 4),
        3 => set_lba48(&mut w, LBA48_LIMIT + xorshift(s) % 4),
        4 => set_lba48(&mut w, xorshift(s) % 4),
        _ => {}
    }
    w
}

/// The rule restated from the ATA words, apart from the driver's source.
fn rule(w: &Block) -> Result<u64, Refusal> {
    let valid = |x: u16| x >> 14 == 0b01;
    let lba48_on = valid(w[83]) && w[83] & 0x400 != 0 && valid(w[87]) && w[86] & 0x400 != 0;
    if !lba48_on {
        return Err(Refusal::No48BitAddress);
    }
    let long = valid(w[106]) && w[106] & 0x1000 != 0;
    let sector_bytes = if long { 2 * (u64::from(w[117]) | (u64::from(w[118]) << 16)) } else { 512 };
    if sector_bytes != 512 {
        return Err(Refusal::SectorSize);
    }
    let n = (0..4).fold(0u64, |acc, i| acc | (u64::from(w[100 + i]) << (16 * i)));
    if n == 0 {
        return Err(Refusal::NoCapacity);
    }
    if n >= LBA48_LIMIT {
        return Err(Refusal::PastLba48);
    }
    Ok(n)
}

#[test]
fn a_hostile_block_is_refused_or_served_by_every_rule() {
    let (mut served, mut no48, mut sector, mut empty, mut past) = (0u64, 0u64, 0u64, 0u64, 0u64);
    for seed in 1..rounds(200_000) {
        let mut s = seeded(seed);
        let w = hostile_block(&mut s);
        let got = capacity(&w);
        assert_eq!(got, rule(&w), "seed {seed}");
        match got {
            Ok(n) => {
                assert!(n >= 1, "seed {seed}: a served disk has a sector");
                assert!(n < LBA48_LIMIT, "seed {seed}: every served LBA fits a FIS");
                served += 1;
            }
            Err(Refusal::No48BitAddress) => no48 += 1,
            Err(Refusal::SectorSize) => sector += 1,
            Err(Refusal::NoCapacity) => empty += 1,
            Err(Refusal::PastLba48) => past += 1,
        }
    }
    if !cfg!(miri) {
        let reached = [
            (served, "served"),
            (no48, "no 48-bit"),
            (sector, "sector size"),
            (empty, "empty"),
            (past, "past 2^48"),
        ];
        for (count, what) in reached {
            assert!(count > 1_000, "the generator reached {what} only {count} times");
        }
    }
}

#[test]
fn words_the_rule_does_not_name_never_change_the_answer() {
    for seed in 1..rounds(50_000) {
        let mut s = seeded(seed);
        let w = hostile_block(&mut s);
        let mut scrambled = w;
        for (i, x) in scrambled.iter_mut().enumerate() {
            if !NAMED.contains(&i) {
                *x = xorshift(&mut s) as u16;
            }
        }
        assert_eq!(capacity(&scrambled), capacity(&w), "seed {seed}");
    }
}
