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

//! the row: MkMmioMap refuses pages over RAM, the kernel image or an
//! IOMMU's registers. The ranges are gathered in the kernel
//! (forbidden_ranges.rs); the rule and the RAM types are proven here.

use super::mmio_forbidden::{is_ram, page_span, touches};

const RAM: (u64, u64) = (0x10_0000, 0x8000_0000);
const KERNEL: (u64, u64) = (0x20_0000, 0x40_0000);
const VTD: (u64, u64) = (0xFED9_0000, 0xFED9_1000);

#[test]
fn an_mmio_grant_never_covers_ram_the_kernel_or_an_iommu() {
    let f = [RAM, KERNEL, VTD];
    assert!(touches(0x7FFF_F000, 0x8000_1000, &f), "a BAR straddling RAM's end");
    assert!(touches(0x30_0000, 0x30_1000, &f), "a page of the kernel image");
    assert!(touches(0xFED9_0000, 0xFED9_1000, &f), "the VT-d unit's registers");
    assert!(touches(0xFED8_F000, 0xFEDA_0000, &f), "a BAR around the unit");
    assert!(touches(0, u64::MAX, &f), "the whole space");
    assert!(!touches(0xFEBC_0000, 0xFEBE_0000, &f), "a NIC's BAR");
    assert!(!touches(0x8000_0000, 0x8000_1000, &f), "the page just past RAM");
    assert!(!touches(0xFED9_1000, 0xFED9_2000, &f), "the page just past the unit");
}

#[test]
fn empty_inverted_and_wrapped_ranges_touch_nothing_by_accident() {
    assert!(!touches(0x1000, 0x1000, &[RAM]), "an empty request");
    assert!(!touches(0x2000, 0x1000, &[(0, u64::MAX)]), "an inverted request");
    assert!(!touches(0x20_0000, 0x20_1000, &[(0x40_0000, 0x20_0000)]), "an inverted range");
}

#[test]
fn only_reserved_and_mmio_descriptors_are_not_ram() {
    for t in [0, 11, 12] {
        assert!(!is_ram(t), "type {t}");
    }
    for t in (1..=10).chain(13..=14).chain([15, 0x7000_0000, 0x8000_0000, u32::MAX]) {
        assert!(is_ram(t), "type {t}: unknown types count as RAM, refused");
    }
}

#[test]
fn a_descriptor_that_overflows_is_refused_not_wrapped() {
    assert_eq!(page_span(0x1000, 2), Some((0x1000, 0x3000)));
    assert_eq!(page_span(0, 0), Some((0, 0)));
    assert_eq!(page_span(u64::MAX - 0xFFF, 1), None, "past the top");
    assert_eq!(page_span(0, u64::MAX), None, "a page count that overflows");
}
