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

use super::budget::{Budget, LINES_PER_POLL};
use super::reason::{bdf_text, is_interrupt, reason_text};

#[test]
fn device_reads_as_lspci_prints_it() {
    assert_eq!(&bdf_text(0x0000), b"00:00.0");
    // 00:14.0, the xHCI controller on most Intel client chipsets.
    assert_eq!(&bdf_text(0x14 << 3), b"00:14.0");
    assert_eq!(&bdf_text(0x02 << 8), b"02:00.0");
    assert_eq!(&bdf_text((0x1F << 3) | 6), b"00:1f.6");
    assert_eq!(&bdf_text(0xFFFF), b"ff:1f.7");
}

/* VT-d 3.4 Appendix A: 0x01 to 0x0D are DMA remapping reasons in legacy
mode, 0x20 to 0x26 interrupt remapping reasons. */
#[test]
fn every_listed_reason_has_its_own_name() {
    let listed: Vec<u8> = (0x01..=0x0D).chain(0x20..=0x26).collect();
    let unlisted = reason_text(0xFF);
    for (i, a) in listed.iter().enumerate() {
        assert_ne!(reason_text(*a), unlisted, "reason {a:#x} unnamed");
        for b in &listed[i + 1..] {
            assert_ne!(reason_text(*a), reason_text(*b), "{a:#x} and {b:#x} share a name");
        }
    }
    assert_eq!(reason_text(0x05), b"page not writable");
    assert_eq!(reason_text(0x06), b"page not readable");
    assert_eq!(reason_text(0x00), unlisted);
}

#[test]
fn interrupt_reasons_are_told_apart_from_dma_reasons() {
    assert!((0x20..0x30).all(is_interrupt));
    assert!(!(0x00..0x20).any(is_interrupt));
    assert!(!(0x30..=0xFF).any(is_interrupt));
}

#[test]
fn a_fault_storm_prints_a_few_lines_and_counts_the_rest() {
    let mut budget = Budget::default();
    let shown = (0..10_000).filter(|_| budget.admit()).count();
    assert_eq!(shown as u32, LINES_PER_POLL);
    assert_eq!(budget.shown, LINES_PER_POLL);
    assert_eq!(budget.hidden, 10_000 - LINES_PER_POLL);
    let mut quiet = Budget::default();
    assert!(quiet.admit());
    assert_eq!(quiet, Budget { shown: 1, hidden: 0 });
}
