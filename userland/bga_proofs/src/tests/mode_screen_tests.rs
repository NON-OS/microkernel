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

//! The virtual screen a mode set leaves: as wide as the mode, scrolled to
//! its origin, so what bga copies at row y, column x is what is shown there.

use nonos_devmodel::FakeBar;

use crate::constants::{DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_X_OFFSET, DISPI_INDEX_Y_OFFSET};
use crate::dispi::{dispi_off, set_mode};
use crate::regs::Regs;

#[test]
fn a_mode_set_leaves_the_virtual_screen_its_own_width_at_the_origin() {
    /*
     * Firmware may leave a wider virtual screen or a panned origin; kept,
     * either shears or shifts every frame by an amount nothing reports.
     */
    let bar = FakeBar::new(0x600);
    bar.present16(dispi_off(DISPI_INDEX_VIRT_WIDTH) as usize, 2048);
    bar.present16(dispi_off(DISPI_INDEX_X_OFFSET) as usize, 16);
    bar.present16(dispi_off(DISPI_INDEX_Y_OFFSET) as usize, 32);
    set_mode(Regs::new(bar.base()), 1280, 800);
    assert_eq!(bar.wrote16(dispi_off(DISPI_INDEX_VIRT_WIDTH) as usize), 1280);
    assert_eq!(bar.wrote16(dispi_off(DISPI_INDEX_X_OFFSET) as usize), 0);
    assert_eq!(bar.wrote16(dispi_off(DISPI_INDEX_Y_OFFSET) as usize), 0);
}
