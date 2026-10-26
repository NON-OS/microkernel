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

use super::place_clear::place_clear;
use super::reserved_rule::{verdict, Held, Verdict};
use super::space::{INTERRUPT_WINDOW_END, INTERRUPT_WINDOW_START, IOVA_BASE, IOVA_LIMIT};

const STOLEN: (u64, u64) = (0x7B80_0000, 0x7F80_0000);

fn held(base: u64, end: u64, device: u64) -> Held {
    Held { base, end, device }
}

#[test]
fn a_region_is_mapped_only_clear_of_ram_grants_the_window_and_others() {
    let (b, e) = STOLEN;
    assert_eq!(verdict(b, e, IOVA_BASE, &[], false), Verdict::Map, "no grant yet");
    assert!(matches!(verdict(b, e, IOVA_BASE, &[], true), Verdict::Refuse(_)), "RAM or held");
    assert!(matches!(verdict(b, e, b + 0x1000, &[], false), Verdict::Refuse(_)), "grants");
    assert_eq!(verdict(b, e, b, &[], false), Verdict::Map, "grants end where it starts");
    let w = (INTERRUPT_WINDOW_START - 0x1000, INTERRUPT_WINDOW_START + 0x1000);
    assert!(matches!(verdict(w.0, w.1, IOVA_BASE, &[], false), Verdict::Refuse(_)));
    assert!(matches!(verdict(b, b, IOVA_BASE, &[], false), Verdict::Refuse(_)), "empty");
    // Below the first grant's address it never meets one.
    assert_eq!(verdict(0xE_0000, 0x10_0000, 0x4000_0000, &[], false), Verdict::Map);
}

#[test]
fn the_same_region_for_a_second_device_is_held_and_a_partial_one_refused() {
    let (b, e) = STOLEN;
    let h = [held(b, e, 1)];
    assert_eq!(verdict(b, e, IOVA_BASE, &h, false), Verdict::Held);
    assert!(matches!(verdict(b + 0x1000, e, IOVA_BASE, &h, false), Verdict::Refuse(_)));
    assert!(matches!(verdict(b - 0x1000, b + 0x1000, IOVA_BASE, &h, false), Verdict::Refuse(_)));
    assert_eq!(verdict(e, e + 0x1000, IOVA_BASE, &h, false), Verdict::Map, "adjacent");
}

#[test]
fn no_grant_is_ever_placed_on_a_held_region() {
    let (b, e) = STOLEN;
    let h = [held(0x20_0000, 0x30_0000, 1), held(0x30_0000, 0x31_0000, 2), held(b, e, 3)];
    let mut next = IOVA_BASE;
    for length in [0x1000u64, 0x10_0000, 0x20_0000, 0x7000_0000, 0x1000, 0x40_0000] {
        let Some((start, end)) = place_clear(next, length, &h) else { break };
        assert!(start >= next && end - start == length, "{start:#x}");
        assert!(h.iter().all(|r| end <= r.base || start >= r.end), "{start:#x} hits a region");
        assert!(end <= INTERRUPT_WINDOW_START || start >= INTERRUPT_WINDOW_END);
        next = end;
    }
    assert_eq!(place_clear(IOVA_BASE, 0x20_0000, &h), Some((0x31_0000, 0x51_0000)));
    assert_eq!(place_clear(IOVA_BASE, IOVA_LIMIT, &h), None, "never past the limit");
}

#[test]
fn the_kernel_places_every_grant_past_its_rmrr_regions_and_reserved_ranges() {
    let table = include_str!("../../../../src/hardware/broker/confine/table.rs");
    assert!(table.contains("let rmrr = self.rmrr.iter().map(|h| (h.base, h.end));"));
    assert!(table.contains("rmrr.chain(tree).collect()"), "both, not one of them");
    let iova = include_str!("../../../../src/hardware/broker/confine/iova.rs");
    assert!(iova.contains("let spans = c.reserved_spans();"));
    assert!(iova.contains("place_avoiding(c.next_iova, length, &spans)?"));
}
