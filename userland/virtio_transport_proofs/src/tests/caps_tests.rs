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

//! The capability parse: what QEMU's layout yields, and which capabilities
//! a hostile layout cannot get taken.

use super::space::{
    qemu_modern, qemu_modern_bars, Space, MODERN_NET, QEMU_MODERN_BAR, QEMU_MSIX_BAR,
};
use crate::caps::{parse, ModernCaps, Region, CFG_COMMON, CFG_DEVICE, CFG_ISR, CFG_NOTIFY};
use crate::pci::{BarInfo, Bars};

fn region(bar: u8, offset: u32, length: u32) -> Option<Region> {
    Some(Region { bar, offset, length })
}

fn parsed(s: &Space) -> ModernCaps {
    parse(&s.cfg(), &qemu_modern_bars())
}

/// A list holding one common configuration, then whatever `extra` adds.
fn with_common(extra: impl FnOnce(&mut Space)) -> ModernCaps {
    let mut s = Space::new(MODERN_NET, 0x40);
    s.cap(0x40, 0x50, CFG_COMMON, 4, 0, 0x1000);
    extra(&mut s);
    parsed(&s)
}

#[test]
fn qemu_modern_net_yields_all_four_structures_in_bar4() {
    let caps = parsed(&qemu_modern(MODERN_NET));
    assert_eq!(caps.common, region(QEMU_MODERN_BAR, 0x0000, 0x1000));
    assert_eq!(caps.isr, region(QEMU_MODERN_BAR, 0x1000, 0x1000));
    assert_eq!(caps.device, region(QEMU_MODERN_BAR, 0x2000, 0x1000));
    assert_eq!(caps.notify, region(QEMU_MODERN_BAR, 0x3000, 0x1000));
    assert_eq!(caps.notify_multiplier, 4);
    let msix = caps.msix.expect("the MSI-X capability is read");
    assert_eq!((msix.table_bar, msix.table), (QEMU_MSIX_BAR, (0, 4 * 16)));
    assert_eq!((msix.pba_bar, msix.pba), (QEMU_MSIX_BAR, (0x800, 0x808)));
}

#[test]
fn structures_in_another_memory_bar_are_found_there() {
    // A display function keeps a framebuffer in BAR0, so its virtio
    // structures sit in another BAR (BAR2 here) and MSI-X in yet another.
    // Nothing in the parse assumes BAR4.
    let bars: Bars = [
        BarInfo::mmio(0x0100_0000),
        BarInfo::ABSENT,
        BarInfo::mmio(0x4000),
        BarInfo::ABSENT,
        BarInfo::mmio(0x1000),
        BarInfo::ABSENT,
    ];
    let mut s = Space::new(0x1050, 0x98);
    s.msix(0x98, 0x70, 2, 4, 0, 4, 0x800);
    s.notify(0x70, 0x60, 2, 0x3000, 0x1000, 4);
    s.cap(0x60, 0x50, CFG_DEVICE, 2, 0x2000, 0x1000);
    s.cap(0x50, 0x40, CFG_ISR, 2, 0x1000, 0x1000);
    s.cap(0x40, 0x00, CFG_COMMON, 2, 0x0000, 0x1000);
    let caps = parse(&s.cfg(), &bars);
    assert_eq!(caps.common, region(2, 0, 0x1000));
    assert_eq!(caps.notify, region(2, 0x3000, 0x1000));
    assert_eq!(caps.device, region(2, 0x2000, 0x1000));
    assert_eq!(caps.msix.map(|m| m.table_bar), Some(4));
}

#[test]
fn the_first_usable_capability_of_a_type_wins() {
    let caps = with_common(|s| {
        s.cap(0x50, 0x60, CFG_COMMON, 4, 0x1000, 0x1000);
        s.notify(0x60, 0x74, 4, 0x3000, 0x1000, 4);
        s.notify(0x74, 0x00, 4, 0x2000, 0x1000, 8);
    });
    assert_eq!(caps.common, region(4, 0, 0x1000), "the earlier common cfg");
    assert_eq!(caps.notify, region(4, 0x3000, 0x1000), "the earlier notify");
    assert_eq!(caps.notify_multiplier, 4, "the multiplier comes from the notify taken");
}

#[test]
fn an_unusable_capability_does_not_shadow_a_later_one() {
    // An I/O BAR notify (QEMU's modern-pio-notify) ahead of the MMIO one.
    let mut bars = qemu_modern_bars();
    bars[2] = BarInfo::io(0x4);
    let mut s = Space::new(MODERN_NET, 0x40);
    s.cap(0x40, 0x50, CFG_COMMON, 4, 0, 0x1000);
    s.notify(0x50, 0x64, 2, 0, 4, 0);
    s.notify(0x64, 0x00, 4, 0x3000, 0x1000, 4);
    let caps = parse(&s.cfg(), &bars);
    assert_eq!(caps.notify, region(4, 0x3000, 0x1000));
    assert_eq!(caps.notify_multiplier, 4);
}

#[test]
fn a_capability_naming_an_io_absent_or_impossible_bar_is_skipped() {
    let mut bars: Bars = qemu_modern_bars();
    bars[0] = BarInfo::io(0x20);
    for bar in [0u8, 2, 3, 5, 6, 7, 0xFF] {
        let mut s = Space::new(MODERN_NET, 0x40);
        s.cap(0x40, 0x00, CFG_COMMON, bar, 0, 0x1000);
        assert_eq!(parse(&s.cfg(), &bars).common, None, "bar {bar}");
    }
}

#[test]
fn a_truncated_cap_len_is_not_read_past() {
    // cap_len 12 cannot hold offset and length.
    let mut s = Space::new(MODERN_NET, 0x40);
    s.raw_cap(0x40, 0x00, 12, CFG_COMMON, 4, 0, 0x1000);
    assert_eq!(parsed(&s).common, None);
    // A notify capability with cap_len 16 has no multiplier. The four
    // bytes after it belong to the next capability and must not be taken
    // for one: here they would read as a valid multiplier of 4.
    let caps = with_common(|s| {
        s.raw_cap(0x50, 0x60, 16, CFG_NOTIFY, 4, 0x3000, 0x1000);
        s.put32(0x60, 4);
    });
    assert_eq!(caps.notify, None);
    assert_eq!(caps.notify_multiplier, 0);
}

#[test]
fn a_capability_running_past_config_space_is_skipped() {
    let mut s = Space::new(MODERN_NET, 0xF4);
    s.cap(0xF4, 0x00, CFG_COMMON, 4, 0, 0x1000);
    assert_eq!(parsed(&s).common, None, "0xF4 + 16 is past 256");
    let mut s = Space::new(MODERN_NET, 0xF0);
    s.cap(0xF0, 0x00, CFG_COMMON, 4, 0, 0x1000);
    assert_eq!(parsed(&s).common, region(4, 0, 0x1000), "0xF0 + 16 is exactly 256");
}

#[test]
fn a_region_outside_its_bar_is_skipped() {
    for (offset, length) in [
        (0x3000u32, 0x1001u32),
        (0x4000, 0x1000),
        (0xFFFF_F000, 0x1000),
        (0xFFFF_FFFF, 0xFFFF_FFFF),
        (0x3F00, 0x38),
    ] {
        let mut s = Space::new(MODERN_NET, 0x40);
        s.cap(0x40, 0x00, CFG_COMMON, 4, offset, length);
        let c = parsed(&s).common;
        // 0x3F00 + 0x38 ends inside the BAR; every other one does not.
        assert_eq!(c.is_some(), offset == 0x3F00, "offset {offset:#x} length {length:#x}");
    }
}

#[test]
fn a_region_too_short_or_misaligned_for_its_structure_is_skipped() {
    let caps = with_common(|s| {
        s.cap(0x50, 0x60, CFG_ISR, 4, 0x1000, 0);
        s.cap(0x60, 0x70, CFG_DEVICE, 4, 0x2002, 0x10);
        s.notify(0x70, 0x00, 4, 0x3001, 0x1000, 4);
    });
    assert_eq!(caps.isr, None, "empty ISR");
    assert_eq!(caps.device, None, "device cfg off a 4-byte boundary");
    assert_eq!(caps.notify, None, "notify off a 2-byte boundary");
    let mut s = Space::new(MODERN_NET, 0x40);
    s.cap(0x40, 0x00, CFG_COMMON, 4, 0, 0x37);
    assert_eq!(parsed(&s).common, None, "common cfg shorter than its registers");
}

#[test]
fn notify_multipliers_zero_and_even_powers_of_two_are_taken() {
    for (mult, ok) in
        [(0u32, true), (2, true), (4, true), (0x1000, true), (1, false), (3, false), (6, false)]
    {
        let caps = with_common(|s| {
            s.notify(0x50, 0x00, 4, 0x3000, 0x1000, mult);
        });
        assert_eq!(caps.notify.is_some(), ok, "multiplier {mult}");
        assert_eq!(caps.notify_multiplier, if ok { mult } else { 0 });
    }
}

#[test]
fn a_region_over_the_msix_table_or_pba_is_skipped() {
    // MSI-X shares BAR4 here: table at 0x3000, PBA at 0x3800.
    let mut s = Space::new(MODERN_NET, 0x40);
    s.cap(0x40, 0x50, CFG_COMMON, 4, 0x0000, 0x1000);
    s.notify(0x50, 0x64, 4, 0x3000, 0x100, 4);
    s.cap(0x64, 0x74, CFG_DEVICE, 4, 0x2F00, 0x100);
    s.cap(0x74, 0x84, CFG_ISR, 4, 0x2000, 0x10);
    s.msix(0x84, 0x00, 8, 4, 0x3000, 4, 0x3800);
    let caps = parsed(&s);
    assert_eq!(caps.common, region(4, 0, 0x1000), "clear of the MSI-X pages");
    assert_eq!(caps.isr, region(4, 0x2000, 0x10), "clear of the MSI-X pages");
    assert_eq!(caps.notify, None, "on the table");
    assert_eq!(caps.device, region(4, 0x2F00, 0x100), "ends below the table page");
}

#[test]
fn a_region_sharing_a_page_with_the_table_is_skipped() {
    // The broker maps whole pages: a region one page up from the table's
    // own bytes but in its page would map the table too.
    let mut s = Space::new(MODERN_NET, 0x40);
    s.cap(0x40, 0x50, CFG_COMMON, 4, 0x0000, 0x100);
    s.msix(0x50, 0x00, 4, 4, 0x0800, 4, 0x0C00);
    assert_eq!(parsed(&s).common, None);
}

#[test]
fn other_vendor_capabilities_and_cfg_types_are_ignored() {
    let caps = with_common(|s| {
        s.cap(0x50, 0x60, 5, 0, 0, 0);
        s.cap(0x60, 0x70, 8, 4, 0x1000, 0x1000);
        s.cap(0x70, 0x00, 0, 4, 0x1000, 0x1000);
    });
    assert_eq!(caps.isr, None);
    assert_eq!(caps.device, None);
    assert_eq!(caps.notify, None);
    assert_eq!(caps.common, region(4, 0, 0x1000));
}
