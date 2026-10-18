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

//! The mode firmware left showing, kept: OVMF programs DISPI for its GOP
//! framebuffer, and a driver that set its own default re-moded the adapter
//! under the firmware's scanout. A mode the driver can draw is kept with no
//! mode register written, and video memory is wiped under it all the same.

use super::adapter::{adapter, found, MODE_BYTES, REG_LEN};
use super::attempt::run;
use super::left::showing;
use crate::constants::{DISPI_INDEX_ENABLE, DISPI_INDEX_XRES};
use crate::dispi::dispi_off;

/// An old frame's word, which bring-up's wipe must not leave.
const OLD: u32 = 0x5EC2_E7ED;

#[test]
fn the_mode_firmware_left_showing_is_kept_without_a_write() {
    let a = adapter(REG_LEN, MODE_BYTES);
    showing(&a, 800, 600);
    // The 8-bit DAC bit: the driver's own mode set would clear it.
    a.regs.present16(dispi_off(DISPI_INDEX_ENABLE) as usize, 0x61);
    a.fb.present32((800 * 600 - 1) * 4, OLD);
    let d = run(found(MODE_BYTES)).unwrap_or_else(|e| panic!("refused: {e:?}"));
    let m = d.server.display.mode;
    assert_eq!((m.width, m.height, m.stride), (800, 600, 3200), "the firmware's mode replaced");
    let reg = |index| a.regs.wrote16(dispi_off(index) as usize);
    assert_eq!((reg(DISPI_INDEX_XRES), reg(DISPI_INDEX_ENABLE)), (800, 0x61), "re-moded");
    assert_eq!(a.fb.wrote32((800 * 600 - 1) * 4), 0, "the kept mode was not wiped");
    assert_eq!(a.fb.wrote32(800 * 600 * 4), 0, "cleared past the kept mode");
}
