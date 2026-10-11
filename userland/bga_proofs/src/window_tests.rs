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

//! The mode set through the real register window, at QEMU's offsets.

use nonos_devmodel::FakeBar;

use crate::constants::*;
use crate::dispi::{little_endian, set_mode, Mode};
use crate::regs::Regs;

const XGA: Mode = Mode { width: 1024, height: 768, stride: 4096 };

#[test]
fn every_access_lands_where_qemu_decodes_it() {
    /*
     * Through the real register window: the DISPI block at 0x500, the VGA
     * attribute port at 0x400 (0x3c0 + 0x40), the byte order at 0x604.
     */
    let bar = FakeBar::new(0x1000);
    // SAFETY: the window is 0x1000 bytes, past REG_BAR_MIN, and outlives it.
    let regs = unsafe { Regs::new(bar.base()) };
    bar.present32(QEXT_SIZE as usize, 8);
    assert!(little_endian(&regs, 0x1000));
    assert!(set_mode(&regs, XGA));
    assert_eq!(bar.wrote8(0x400), VGA_ATT_PAS);
    assert_eq!(bar.wrote16(0x502), 1024);
    assert_eq!(bar.wrote16(0x504), 768);
    assert_eq!(bar.wrote16(0x508), DISPI_ENABLED | DISPI_LFB_ENABLED);
    assert_eq!(bar.wrote32(0x604), QEXT_LITTLE_ENDIAN);
}
