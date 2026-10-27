// NONOS Operating System (AGPL-3.0-or-later)
//! Transmit power: the cap is the world domain's lowest 2.4 GHz EIRP less
//! 2 dBi in quarter-dB steps; each rate family's index is the efuse's,
//! its signed half-dB diffs counted twice, never above the cap, the lowest
//! over channels 1 to 13; an unburned base is the cap; and it is written as
//! the 8822C takes it: references per path, diffs per four rates.

use crate::efuse::facts::PathPower;
use crate::model::Chip;
use crate::phy::bits::read32_mask;
use crate::phy::txpower::index::{cap, indexes, Indexes};
use crate::phy::txpower::set_tx_power;
use crate::phy::txpower::write::diff_word;

fn path(cck: u8, bw40: u8, diff: u8) -> PathPower {
    PathPower { cck: [cck; 6], bw40: [bw40; 5], diff }
}

#[test]
fn the_cap_is_eighteen_dbm_in_quarter_db() {
    assert_eq!(cap(), (20 - 2) * 4);
}

#[test]
fn the_efuse_diffs_are_signed_half_db() {
    // OFDM diff +2 (low nibble), HT20 diff -1 (high nibble 0xF).
    let i = indexes(&path(0x30, 0x34, 0xF2), 0x7F);
    assert_eq!(i, Indexes { cck: 0x30, ofdm: 0x38, ht: 0x32 });
}

#[test]
fn no_index_passes_the_cap_and_unburned_is_the_cap() {
    assert_eq!(indexes(&path(0x60, 0x60, 0x00), 72), Indexes { cck: 72, ofdm: 72, ht: 72 });
    assert_eq!(indexes(&path(0xFF, 0xFF, 0x77), 72), Indexes { cck: 72, ofdm: 72, ht: 72 });
    assert_eq!(indexes(&path(0x01, 0x01, 0x88), 72).ofdm, 0, "never below zero");
}

#[test]
fn the_lowest_group_of_channels_1_to_13_is_taken() {
    let mut p = path(0x40, 0x40, 0);
    p.cck = [0x40, 0x3C, 0x40, 0x40, 0x40, 0x10]; // group 5 is channel 14's CCK
    p.bw40 = [0x40, 0x40, 0x40, 0x40, 0x3A];
    assert_eq!(indexes(&p, 0x7F), Indexes { cck: 0x3C, ofdm: 0x3A, ht: 0x3A });
}

#[test]
fn a_diff_word_is_the_smallest_over_the_paths() {
    let a = Indexes { cck: 0, ofdm: 0x40, ht: 0x42 };
    let b = Indexes { cck: 0, ofdm: 0x45, ht: 0x42 };
    assert_eq!(diff_word(&[a, b], |i| i16::from(i.ofdm) - i16::from(i.ht)), 0x7E7E_7E7E);
    assert_eq!(diff_word(&[b], |i| i16::from(i.ofdm) - i16::from(i.ht)), 0x0303_0303);
}

#[test]
fn the_references_and_diffs_land_where_the_8822c_reads_them() {
    let c = Chip::new();
    set_tx_power(&c, &[path(0x30, 0x34, 0x02), path(0x31, 0x35, 0x02)], 2);
    assert_eq!(read32_mask(&c, 0x18A0, 0x7F_0000), 0x30, "path A CCK reference");
    assert_eq!(read32_mask(&c, 0x41A0, 0x7F_0000), 0x31, "path B CCK reference");
    assert_eq!(read32_mask(&c, 0x18E8, 0x1_FC00), 0x34, "path A MCS7 reference");
    assert_eq!(read32_mask(&c, 0x41E8, 0x1_FC00), 0x35);
    assert_eq!(c.at(0x3A04), [0x0404_0404], "6-24M: OFDM over HT, +4 steps");
    assert_eq!((c.at(0x3A00), c.at(0x3A0C)), (vec![0], vec![0]));
    assert!(c.at(0x1C90).iter().all(|v| v & 1 << 15 == 0));
}

#[test]
fn one_path_writes_path_a_alone() {
    let c = Chip::new();
    set_tx_power(&c, &[path(0x30, 0x34, 0), path(0x31, 0x35, 0)], 1);
    assert!(c.at(0x41A0).is_empty() && c.at(0x41E8).is_empty());
}
