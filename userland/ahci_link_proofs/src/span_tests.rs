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

//! What a command may name before it is built: sectors inside the served
//! disk and inside 48 bits, and one PRD entry whose byte count is even, at
//! most 4 MiB and never more than the data buffer the driver owns.

use crate::constants::ata::{DATA_BUF_BYTES, MAX_SECTORS, PRD_MAX_BYTES, SECTOR_SIZE};
use crate::engine::prd_count::{data_bytes, prd_dbc};
use crate::engine::span::within;
use crate::identity::capacity;
use crate::identity_tests::{qemu_block, LBA48_LIMIT};
use crate::rw_parse::parse;

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

fn rw_body(lba: u64, n: u32) -> [u8; 12] {
    let mut b = [0u8; 12];
    b[0..8].copy_from_slice(&lba.to_le_bytes());
    b[8..12].copy_from_slice(&n.to_le_bytes());
    b
}

/// The span rule restated in u128, where nothing wraps.
fn inside(capacity: u64, lba: u64, sectors: u32) -> bool {
    let end = u128::from(lba) + u128::from(sectors);
    sectors != 0 && end <= u128::from(capacity) && end <= u128::from(LBA48_LIMIT)
}

#[test]
fn a_span_must_name_a_sector_and_end_inside_the_disk() {
    let capacity = 1000;
    assert!(!within(capacity, 0, 0), "no sectors");
    assert!(!within(capacity, 999, 0), "no sectors at the last LBA");
    assert!(within(capacity, 0, 1), "the first sector");
    assert!(within(capacity, 999, 1), "the last sector");
    assert!(within(capacity, 936, 64), "a full buffer ending on the last sector");
    assert!(!within(capacity, 937, 64), "a full buffer one sector past the end");
    assert!(!within(capacity, 1000, 1), "the first LBA past the end");
    assert!(!within(0, 0, 1), "a disk with no sectors");
}

#[test]
fn a_span_that_wraps_u64_lies_nowhere() {
    assert!(!within(u64::MAX, u64::MAX, 1));
    assert!(!within(u64::MAX, u64::MAX - 1, 2));
    assert!(!within(u64::MAX, u64::MAX - 62, 64));
}

#[test]
fn a_span_past_48_bits_is_refused_whatever_capacity_says() {
    assert!(within(u64::MAX, LBA48_LIMIT - 1, 1), "the last LBA a FIS carries");
    assert!(!within(u64::MAX, LBA48_LIMIT, 1), "the first LBA a FIS would wrap to 0");
    assert!(!within(u64::MAX, LBA48_LIMIT - 1, 2));
    assert!(!within(u64::MAX, LBA48_LIMIT + 5, 1));
}

#[test]
fn hostile_spans_are_inside_exactly_when_the_rule_says() {
    for seed in 1..rounds(200_000) {
        let mut s = seeded(seed);
        let r = xorshift(&mut s);
        let capacity = match r % 4 {
            0 => xorshift(&mut s) % 2048,
            1 => LBA48_LIMIT - xorshift(&mut s) % 4,
            2 => u64::MAX - xorshift(&mut s) % 4,
            _ => xorshift(&mut s),
        };
        let lba = match (r >> 2) % 4 {
            0 => capacity.wrapping_sub(xorshift(&mut s) % 80),
            1 => u64::MAX - xorshift(&mut s) % 80,
            2 => LBA48_LIMIT - xorshift(&mut s) % 80,
            _ => xorshift(&mut s),
        };
        let sectors = match (r >> 4) % 3 {
            0 => (xorshift(&mut s) % 70) as u32,
            1 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => xorshift(&mut s) as u32,
        };
        assert_eq!(
            within(capacity, lba, sectors),
            inside(capacity, lba, sectors),
            "seed {seed}: capacity {capacity:#x} lba {lba:#x} sectors {sectors}"
        );
    }
}

#[test]
fn every_request_the_parser_passes_on_a_served_disk_is_built_inside_it() {
    /*
     * The parser answers the client; `within` and `data_bytes` gate the
     * command. On any capacity IDENTIFY lets through, the two never
     * disagree: what the client may ask, the engine builds, and no further.
     */
    for seed in 1..rounds(100_000) {
        let mut s = seeded(seed);
        let served = match capacity(&qemu_block(1 + xorshift(&mut s) % (LBA48_LIMIT - 1))) {
            Ok(n) => n,
            Err(e) => panic!("seed {seed}: a QEMU block was refused: {e:?}"),
        };
        let lba = if xorshift(&mut s) & 1 == 0 {
            served.saturating_sub(xorshift(&mut s) % 80)
        } else {
            xorshift(&mut s) % served
        };
        let n = (xorshift(&mut s) % 70) as u32;
        let parsed = parse(&rw_body(lba, n), served);
        let engine = within(served, lba, n) && data_bytes(n).is_some();
        assert_eq!(parsed.is_ok(), engine, "seed {seed}: lba {lba} n {n} on {served}");
    }
}

#[test]
fn the_data_count_never_names_more_than_the_buffer() {
    assert_eq!(data_bytes(0), None, "no sectors");
    assert_eq!(data_bytes(1), Some(512));
    assert_eq!(data_bytes(MAX_SECTORS), Some(DATA_BUF_BYTES as u32), "a full buffer");
    assert_eq!(data_bytes(MAX_SECTORS + 1), None, "one sector past the buffer");
    assert_eq!(data_bytes(u32::MAX), None);
    assert_eq!(data_bytes(u32::MAX / SECTOR_SIZE as u32 + 1), None, "a count whose bytes wrap u32");
    for sectors in 0..=MAX_SECTORS + 8 {
        match data_bytes(sectors) {
            Some(b) => {
                assert!((1..=MAX_SECTORS).contains(&sectors), "{sectors} sectors built");
                assert_eq!(u64::from(b), u64::from(sectors) * SECTOR_SIZE as u64);
                assert!(u64::from(b) <= DATA_BUF_BYTES);
            }
            None => assert!(sectors == 0 || sectors > MAX_SECTORS, "{sectors} sectors refused"),
        }
    }
}

#[test]
fn a_prd_count_is_nonzero_even_and_at_most_4_mib() {
    assert_eq!(prd_dbc(0), None, "zero would wrap to a 4 MiB count with the interrupt bit");
    assert_eq!(prd_dbc(1), None, "odd");
    assert_eq!(prd_dbc(511), None, "odd");
    assert_eq!(prd_dbc(2), Some(1), "the smallest count");
    assert_eq!(prd_dbc(512), Some(511));
    assert_eq!(prd_dbc(PRD_MAX_BYTES), Some(0x3f_ffff), "the largest count");
    assert_eq!(prd_dbc(PRD_MAX_BYTES + 2), None, "past 22 bits");
    assert_eq!(prd_dbc(u32::MAX), None);
    assert_eq!(prd_dbc(u32::MAX - 1), None);
}

#[test]
fn every_prd_count_fits_the_field_whatever_the_bytes() {
    let mut s = seeded(7);
    for i in 0..rounds(200_000) {
        let bytes = match i % 3 {
            0 => xorshift(&mut s) as u32,
            1 => (xorshift(&mut s) % (u64::from(PRD_MAX_BYTES) + 8)) as u32,
            _ => (xorshift(&mut s) % 70) as u32 * SECTOR_SIZE as u32,
        };
        match prd_dbc(bytes) {
            Some(dbc) => {
                assert_eq!(dbc + 1, bytes);
                assert_eq!(dbc & 1, 1, "{bytes}: DBC bit 0 must read 1");
                assert!(dbc < 1 << 22, "{bytes}: DBC past 22 bits or the interrupt bit set");
            }
            None => assert!(bytes == 0 || bytes & 1 == 1 || bytes > PRD_MAX_BYTES, "{bytes}"),
        }
    }
    for sectors in 0..=u32::from(u16::MAX) {
        if let Some(b) = data_bytes(sectors) {
            assert!(prd_dbc(b).is_some(), "{sectors} sectors give a count the PRD refuses");
        }
    }
}
