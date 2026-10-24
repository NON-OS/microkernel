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

//! A mode the driver sets is shown: the VGA attribute controller's palette
//! address source is set, port 0x3C0 at BAR2 + 0x400 (as observed of
//! QEMU's adapter, unverified).

use nonos_devmodel::FakeBar;

use crate::constants::VGA_ATT_PAS;
use crate::dispi::set_mode;
use crate::regs::Regs;

#[test]
fn a_mode_set_leaves_the_screen_shown() {
    /*
     * With the palette address source clear the adapter scans out nothing:
     * every register reads back right and the screen stays black.
     */
    let bar = FakeBar::new(0x600);
    set_mode(Regs::new(bar.base()), 1024, 768);
    assert_eq!(bar.wrote8(0x400), VGA_ATT_PAS);
}
