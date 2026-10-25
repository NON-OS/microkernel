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

//! The mode set through the shipping register window, at QEMU's offsets and
//! in order: the screen unblanked before any DISPI register, the adapter
//! disabled while the mode is written, ENABLE last but for the line layout.
//! A mode written with ENABLE on shows a mode nobody asked for, which the
//! settled registers cannot show; the shim's hook sees each 16-bit write.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_devmodel::FakeBar;
use nonos_libc::on_w16;

use crate::constants::{DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BANK, DISPI_INDEX_BPP};
use crate::constants::{DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_XRES};
use crate::constants::{DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET, VGA_ATT_PAS};
use crate::constants::{DISPI_LFB_ENABLED, DISPI_NOCLEARMEM, QEXT_LITTLE_ENDIAN, QEXT_SIZE};
use crate::dispi::{dispi_off, little_endian, set_mode};
use crate::regs::Regs;

const ON: u16 = DISPI_ENABLED | DISPI_LFB_ENABLED | DISPI_NOCLEARMEM;

#[test]
fn every_access_lands_where_qemu_decodes_it() {
    // DISPI at 0x500, the attribute port at 0x400 (0x3C0 + 0x40), byte order at 0x604.
    let bar = FakeBar::new(0x1000);
    bar.present32(QEXT_SIZE as usize, 8);
    let regs = Regs::new(bar.base());
    assert!(little_endian(regs, 0x1000));
    set_mode(regs, 1024, 768);
    assert_eq!(bar.wrote8(0x400), VGA_ATT_PAS);
    assert_eq!((bar.wrote16(0x502), bar.wrote16(0x504), bar.wrote16(0x508)), (1024, 768, ON));
    assert_eq!(bar.wrote32(0x604), QEXT_LITTLE_ENDIAN);
}

#[test]
fn the_screen_is_unblanked_then_the_mode_written_disabled_and_enabled_last() {
    let bar = Rc::new(FakeBar::new(0x1000));
    let log = Rc::new(RefCell::new(Vec::new()));
    let block = bar.base() + dispi_off(0) as u64;
    let (seen, window) = (Rc::clone(&log), Rc::clone(&bar));
    on_w16(move |addr, value| {
        let shown = window.wrote8(0x400) == VGA_ATT_PAS;
        seen.borrow_mut().push((((addr - block) / 2) as u32, value, shown));
    });
    set_mode(Regs::new(bar.base()), 1024, 768);
    let want = [
        (DISPI_INDEX_ENABLE, 0),
        (DISPI_INDEX_BANK, 0),
        (DISPI_INDEX_XRES, 1024),
        (DISPI_INDEX_YRES, 768),
        (DISPI_INDEX_BPP, DISPI_BPP_32),
        (DISPI_INDEX_ENABLE, ON),
        (DISPI_INDEX_VIRT_WIDTH, 1024),
        (DISPI_INDEX_X_OFFSET, 0),
        (DISPI_INDEX_Y_OFFSET, 0),
    ];
    assert_eq!(log.borrow().iter().map(|&(i, v, _)| (i, v)).collect::<Vec<_>>(), want);
    assert!(log.borrow().iter().all(|w| w.2), "a DISPI register written before the unblank");
}
