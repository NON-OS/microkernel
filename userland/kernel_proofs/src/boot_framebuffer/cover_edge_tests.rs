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

//! BARs the cover rule must read with care (L79): one whose end passes the
//! top of the address space, and those it must not read at all, of no size,
//! of ports, of no kind, or in a slot past the record's count. Addresses are
//! illustrative.

use super::cover_find::{find, mine, Rec, KIB, MIB, MMIO, NONE, PIO};
use super::cover_rule::{holds, pages};
use super::cover_tests::SMALL;

const TOP: u64 = 0xFFFF_FFFF_0000_0000;

#[test]
fn a_bar_whose_end_overflows_reaches_everything_above_its_base() {
    let wild = Rec::new(&[(TOP, u64::MAX, MMIO)], false);
    assert_eq!(pages(TOP, u64::MAX), (TOP, u64::MAX));
    assert!(mine((TOP, SMALL), &wild), "at its base");
    assert!(mine((0xFFFF_FFFF_FFFF_F000, 4 * KIB), &wild), "the last page there is");
    assert!(!mine((TOP - SMALL, SMALL), &wild), "below its base");
    // An end that fits, but whose page round-up would not.
    let near = 0xFFFF_FFFF_FFFF_E000;
    assert_eq!(pages(near, 6 * KIB), (near, u64::MAX));
    let tail = Rec::new(&[(near, 6 * KIB, MMIO)], false);
    assert!(mine((u64::MAX - 0x3FF, 0x100), &tail), "the top page shared");
}

#[test]
fn zero_size_port_kindless_and_uncounted_bars_hold_nothing() {
    let fb = (0xD000_0000, SMALL);
    // Off its page, so its page span would meet the frame were it read.
    assert!(!holds(fb, &[(0xD000_0010, 0, MMIO)], 1, MMIO), "no size");
    assert!(!holds(fb, &[(0xD000_0000, 16 * MIB, PIO)], 1, MMIO), "ports");
    assert!(!holds(fb, &[(0xD000_0000, 16 * MIB, NONE)], 1, MMIO), "no kind");
    let mut bars = [(0, 0, NONE); 6];
    bars[3] = (0xD000_0000, 16 * MIB, MMIO);
    assert!(!holds(fb, &bars, 3, MMIO), "slot 3 with three counted");
    assert!(holds(fb, &bars, 4, MMIO), "slot 3 with four counted");
    bars[3] = (0, 0, NONE);
    bars[5] = (0xD000_0000, 16 * MIB, MMIO);
    assert!(!holds(fb, &bars, 5, MMIO) && !holds(fb, &bars, 0, MMIO), "slot 5 uncounted");
    assert!(holds(fb, &bars, 6, MMIO), "slot 5 with all six counted");
}

#[test]
fn a_frame_in_another_records_bar_is_not_that_records() {
    let fb = (0xD000_0000, SMALL);
    // A function that is no display, whose BAR0 the frame lies in, and the
    // one display, whose memory is elsewhere.
    let other = Rec::new(&[(0xD000_0000, 16 * MIB, MMIO)], false);
    let display = Rec::new(&[(0xFD00_0000, 16 * MIB, MMIO)], true);
    assert!(!mine(fb, &display), "the display's own BARs");
    assert_eq!(find(fb, &[display, other]), vec![1], "the holder only");
    let first = Rec::new(&[(0xD000_0000, 16 * MIB, MMIO)], true);
    let second = Rec::new(&[(0xFD00_0000, 16 * MIB, MMIO)], true);
    assert_eq!(find(fb, &[first, second]), vec![0], "of two displays, the holder");
}
