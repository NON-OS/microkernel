// NONOS Operating System (AGPL-3.0-or-later)
//! Damaged tables: refused or read, never a panic, an overflow (the proofs
//! run with overflow checks on) or a read outside the slice.

use crate::arch::aarch64::iort::{root_complexes, IortError};
use crate::records_tests::its;
use crate::table::Table;

fn sample() -> Vec<u8> {
    let mut t = Table::new();
    let group = t.its_group(&[0]);
    let maps = [(0, 1, 0, group, 1), (0, 0x1_0000, 0, group, 0)];
    let smmu = t.smmu(0x0905_0000, 0x11, [1, 2, 3, 4], 0, &maps);
    t.root(0, &[(0, 0x8000, 0, smmu, 0), (0x8000, 0x8000, 0, group, 0)]);
    t.finish()
}

#[test]
fn the_sample_reads() {
    assert!(root_complexes(&sample(), its).unwrap()[0].is_some());
}

#[test]
fn every_truncation() {
    let table = sample();
    for end in 0..table.len() {
        let mut cut = table[..end].to_vec();
        assert!(root_complexes(&cut, its).is_err(), "cut at {end}");
        // The header still claiming the whole table changes nothing.
        if cut.len() >= 8 {
            cut[4..8].copy_from_slice(&(end as u32).to_le_bytes());
            let _ = root_complexes(&cut, its);
        }
    }
}

#[test]
fn every_byte_at_every_hostile_value() {
    let table = sample();
    for at in 0..table.len() {
        for value in [0x00, 0x01, 0x7F, 0x80, 0xFE, 0xFF, table[at].wrapping_add(1)] {
            let mut bad = table.clone();
            bad[at] = value;
            let _ = root_complexes(&bad, its);
        }
    }
}

#[test]
fn a_signature_or_length_that_lies() {
    let mut table = sample();
    table[0] = b'X';
    assert_eq!(root_complexes(&table, its), Err(IortError::BadNode));
    let mut table = sample();
    table[4..8].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(root_complexes(&table, its), Err(IortError::Short));
}
