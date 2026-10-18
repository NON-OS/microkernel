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

//! Reading back the mode DISPI shows: what bga keeps from firmware at
//! start, and how it checks a mode set took.

use crate::constants::{DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_WIDTH};
use crate::constants::{DISPI_INDEX_XRES, DISPI_INDEX_X_OFFSET, DISPI_INDEX_Y_OFFSET};
use crate::dispi::{current, dispi_off, set_mode, DispiMode};
use crate::model::Adapter;

fn poke(a: &Adapter, index: u32, value: u16) {
    a.bar.present16(dispi_off(index) as usize, value);
}

#[test]
fn an_adapter_left_off_shows_no_mode() {
    assert_eq!(current(Adapter::new().regs()), None);
}

#[test]
fn a_mode_set_reads_back_as_the_mode_with_four_bytes_a_pixel() {
    let a = Adapter::new();
    set_mode(a.regs(), 1280, 800);
    let want = DispiMode { width: 1280, height: 800, stride: 1280 * 4 };
    assert_eq!(current(a.regs()), Some(want));
}

#[test]
fn a_firmware_mode_wider_in_memory_than_on_screen_keeps_its_stride() {
    // Firmware may scan out lines wider than the visible ones; a copy at
    // the visible width would shear every row after the first.
    let a = Adapter::new();
    set_mode(a.regs(), 1024, 768);
    poke(&a, DISPI_INDEX_VIRT_WIDTH, 1280);
    assert_eq!(current(a.regs()).map(|m| m.stride), Some(1280 * 4));
}

#[test]
fn a_mode_not_32_bits_or_without_its_framebuffer_is_none_bga_can_use() {
    let a = Adapter::new();
    set_mode(a.regs(), 1024, 768);
    poke(&a, DISPI_INDEX_BPP, 16);
    assert_eq!(current(a.regs()), None, "16 bpp is not ARGB8888");
    set_mode(a.regs(), 1024, 768);
    poke(&a, DISPI_INDEX_ENABLE, 0x01);
    assert_eq!(current(a.regs()), None, "no linear framebuffer decoding");
    set_mode(a.regs(), 1024, 768);
    poke(&a, DISPI_INDEX_XRES, 0);
    assert_eq!(current(a.regs()), None, "a zero-wide mode");
}

#[test]
fn a_panned_mode_is_none_bga_draws_into() {
    // Every copy writes from the start of video memory, so a pan shears it.
    let a = Adapter::new();
    for index in [DISPI_INDEX_X_OFFSET, DISPI_INDEX_Y_OFFSET] {
        set_mode(a.regs(), 1024, 768);
        poke(&a, index, 1);
        assert_eq!(current(a.regs()), None, "panned at index {index}");
    }
}
