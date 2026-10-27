// NONOS Operating System (AGPL-3.0-or-later)
//! One DC measurement is bounded by count as rtw88 bounds it: at most 10000
//! reads to find 100 samples in range, then at most 101 rounds re-reading
//! the two ends; and it measures: a steady DC is its value, either sign.

use std::cell::Cell;

use crate::model::Chip;
use crate::phy::dack::search::{measure, read_sample};

/// rtw88's bounds, written out here so a change to the driver's shows.
const SAMPLE_READS: u32 = 10_000;
const SEARCH_ROUNDS: u32 = 100;

#[test]
fn a_steady_dc_is_measured_in_100_reads() {
    let reads = Cell::new(0);
    let r = measure(|| {
        reads.set(reads.get() + 1);
        (0x003, 0x3FD)
    });
    assert_eq!(r, (0x003, 0x3FD));
    assert_eq!(reads.get(), 100);
}

#[test]
fn samples_out_of_range_end_at_10000_reads() {
    // Out of range for exactly the bound, zero after: the fill gives up with
    // one stale sample, and one search round re-reads it away.
    let reads = Cell::new(0u32);
    let r = measure(|| {
        reads.set(reads.get() + 1);
        if reads.get() <= SAMPLE_READS {
            (0x100, 0x100)
        } else {
            (0, 0)
        }
    });
    assert_eq!(reads.get(), SAMPLE_READS + 2);
    assert_eq!(r, (0, 0));
}

#[test]
fn a_spread_that_never_settles_ends_after_101_rounds() {
    let reads = Cell::new(0u32);
    measure(|| {
        reads.set(reads.get() + 1);
        if reads.get() % 2 == 0 {
            (0x00, 0x00)
        } else {
            (0x60, 0x00)
        }
    });
    assert_eq!(reads.get(), 100 + 2 * (SEARCH_ROUNDS + 1));
}

#[test]
fn the_sample_register_is_split_into_i_and_q() {
    let c = Chip::new();
    *c.hw.sample.borrow_mut() = Some(Box::new(|| 0xFFC0_3123));
    assert_eq!(read_sample(&c), (0x003, 0x123));
    assert_eq!(c.hw.sample_reads.get(), 1);
}
