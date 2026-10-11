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

//! The mode set, write by write, against bochs_hw_setmode.

use crate::constants::*;
use crate::dispi::{set_mode, Mode};
use crate::model::{Model, Op};

const XGA: Mode = Mode { width: 1024, height: 768, stride: 4096 };

#[test]
fn the_registers_are_written_in_linux_order_enable_last() {
    /*
     * The adapter latches a mode on ENABLE. Writing it before XRES, or
     * leaving it on while the mode changes, shows a mode nobody asked for
     * for a frame or for good; a passive window cannot see that, the
     * recording model does.
     */
    let m = Model::default();
    assert!(set_mode(&m, XGA));
    let d = |i, v| Op::Dispi(i, v);
    let want = vec![
        Op::VgaRead(VGA_IS1_RC),
        Op::Vga(VGA_ATT_W, VGA_ATT_PAS),
        d(DISPI_INDEX_ENABLE, 0),
        d(DISPI_INDEX_BPP, 32),
        d(DISPI_INDEX_XRES, 1024),
        d(DISPI_INDEX_YRES, 768),
        d(DISPI_INDEX_BANK, 0),
        d(DISPI_INDEX_VIRT_WIDTH, 1024),
        d(DISPI_INDEX_VIRT_HEIGHT, 768),
        d(DISPI_INDEX_X_OFFSET, 0),
        d(DISPI_INDEX_Y_OFFSET, 0),
        d(DISPI_INDEX_ENABLE, DISPI_ENABLED | DISPI_LFB_ENABLED),
    ];
    assert_eq!(*m.log.borrow(), want);
}

#[test]
fn a_mode_the_adapter_does_not_take_is_reported_refused() {
    // QEMU reads a mode past its limits back clamped.
    for (i, v) in [(DISPI_INDEX_XRES, 800), (DISPI_INDEX_YRES, 600), (DISPI_INDEX_BPP, 16)] {
        let m = Model { stuck: Some((i, v)), ..Model::default() };
        assert!(!set_mode(&m, XGA), "index {i} stuck at {v} passed as set");
    }
}

#[test]
fn the_virtual_width_follows_the_stride_not_the_width() {
    let m = Model::default();
    assert!(set_mode(&m, Mode { width: 1000, height: 600, stride: 1024 * 4 }));
    assert_eq!(m.regs.borrow()[DISPI_INDEX_VIRT_WIDTH as usize], 1024);
}
