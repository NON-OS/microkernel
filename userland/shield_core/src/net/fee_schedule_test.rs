// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::indexing_slicing)]

use super::{fee, schedule, split, Schedule, RUNG};

/// The production policy's schedules on Sepolia, read on 1 October.
const ETH: Schedule = Schedule {
    protocol: 500_000_000_000_000,
    ladder: [
        2_500_000_000_000_000,
        5_000_000_000_000_000,
        10_000_000_000_000_000,
        20_000_000_000_000_000,
    ],
};
const NOX: Schedule = Schedule {
    protocol: 400_000_000_000,
    ladder: [2_000_000_000_000, 4_000_000_000_000, 8_000_000_000_000, 16_000_000_000_000],
};

/// The four pinned 37-limb wallet vectors, each fee checked on chain with `settlementFee`.
#[test]
fn the_fees_are_the_pinned_vectors_fees() {
    assert_eq!(fee(ETH, 0, 50, 0), Some(3_000_000_000_000_000));
    assert_eq!(fee(ETH, 0, 50, 10_000_000_000_000_000), Some(2_550_000_000_000_000));
    assert_eq!(fee(NOX, 0, 50, 0), Some(2_400_000_000_000));
    assert_eq!(fee(NOX, 0, 50, 10_000_000_000_000), Some(2_050_000_000_000));
}

/// A fee is the protocol part plus one rung, so there is no fifth rung to pick.
#[test]
fn a_fee_off_the_ladder_is_never_built() {
    assert_eq!(fee(ETH, 4, 50, 0), None);
}

#[test]
fn every_spend_pays_the_lowest_rung() {
    assert_eq!(RUNG, 0);
    assert_eq!(fee(ETH, RUNG, 50, 0), Some(3_000_000_000_000_000));
    assert_eq!(fee(NOX, RUNG, 50, 0), Some(2_400_000_000_000));
}

#[test]
fn an_unset_schedule_is_refused() {
    let mut reply = vec![0u8; 192];
    reply[31] = 5;
    assert!(schedule(&reply).is_err());
    reply[191] = 1;
    assert_eq!(schedule(&reply).expect("a schedule").protocol, 5);
}

/// The two parts a screen names: the flat or 0.50% protocol part, and one rung of the ladder.
#[test]
fn the_split_names_the_protocol_part_and_the_rung() {
    assert_eq!(split(ETH, 0, 50, 0), Some((500_000_000_000_000, 2_500_000_000_000_000)));
    let withdrawn = split(ETH, 2, 50, 10_000_000_000_000_000);
    assert_eq!(withdrawn, Some((50_000_000_000_000, 10_000_000_000_000_000)));
    assert_eq!(split(NOX, 3, 50, 0), Some((400_000_000_000, 16_000_000_000_000)));
    assert_eq!(split(NOX, 0, 50, 10_000_000_000_000), Some((50_000_000_000, 2_000_000_000_000)));
    let wide = Schedule { protocol: u64::MAX, ladder: [1, 2, 3, 4] };
    assert_eq!(split(wide, 0, 50, 0), None);
}
