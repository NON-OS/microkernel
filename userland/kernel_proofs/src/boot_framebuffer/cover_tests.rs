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

//! Where a framebuffer lies against one function's BARs (L79), judged on
//! whole pages. Every address and size here is illustrative, picked to show
//! a shape; none is a fact about any real device.

use super::cover_find::{mine, Rec, KIB, MIB, MMIO, NONE, PIO};

/// Frames at four bytes a pixel: 1024x768, 1280x800 and 1920x1080, the first
/// shared with the other shape tests.
pub(super) const SMALL: u64 = 1024 * 768 * 4;
const WIDE: u64 = 1280 * 800 * 4;
const FULL: u64 = 1920 * 1080 * 4;

#[test]
fn a_frame_in_a_vga_style_bar0_is_held() {
    // Illustrative: 16 MiB of prefetchable memory in BAR0, registers in BAR2.
    let bars = [(0xFD00_0000, 16 * MIB, MMIO), (0, 0, NONE), (0xFEBF_0000, 4 * KIB, MMIO)];
    let vga = Rec::new(&bars, true);
    assert!(mine((0xFD00_0000, SMALL), &vga), "at BAR0's start");
    assert!(mine((0xFD00_0000 + 16 * MIB - SMALL, SMALL), &vga), "at BAR0's end");
    assert!(!mine((0xFD00_0000 + 16 * MIB, SMALL), &vga), "just past BAR0");
}

#[test]
fn a_frame_in_a_paravirtual_gpu_bar0_is_held() {
    // Illustrative: 8 MiB of frame memory in BAR0, registers in BAR1, and a
    // 64-bit BAR4 high up, the slot after it left empty (a 64-bit BAR taking
    // two slots is recalled, unverified).
    let bars = [
        (0xFC00_0000, 8 * MIB, MMIO),
        (0xFEBD_0000, 4 * KIB, MMIO),
        (0, 0, NONE),
        (0, 0, NONE),
        (0x8_0000_0000, 16 * KIB, MMIO),
    ];
    let gpu = Rec::new(&bars, true);
    assert!(mine((0xFC00_0000, WIDE), &gpu));
    assert!(mine((0xFC00_0000 + 0x40, WIDE), &gpu), "a first pixel off its page");
    assert!(!mine((0xFE00_0000, WIDE), &gpu), "between the BARs");
}

#[test]
fn a_frame_in_an_aperture_in_bar2_is_held() {
    // Illustrative: registers in a 64-bit BAR0, a 256 MiB graphics aperture
    // in a 64-bit BAR2, ports in BAR4; each 64-bit BAR's second slot left
    // empty (two slots each is recalled, unverified).
    let regs = (0xF600_0000, 16 * MIB, MMIO);
    let aperture = (0xE000_0000, 256 * MIB, MMIO);
    let bars = [regs, (0, 0, NONE), aperture, (0, 0, NONE), (0xF000, 64, PIO)];
    let igpu = Rec::new(&bars, true);
    assert!(mine((0xE000_0000, FULL), &igpu));
    assert!(mine((0xE000_0000 + 0x80_0000, FULL), &igpu), "deeper in the aperture");
    assert!(!mine((0xE000_0000, FULL), &Rec::new(&bars[..2], true)), "the registers alone");
}
