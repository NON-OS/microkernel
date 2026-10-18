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

//! The framebuffer's byte order: set little endian only where the register
//! is, in the mapping the broker made, and nothing written elsewhere.

use nonos_devmodel::FakeBar;

use crate::constants::{QEXT_ENDIAN, QEXT_LITTLE_ENDIAN, QEXT_SIZE};
use crate::dispi::little_endian;
use crate::regs::Regs;

const WINDOW: usize = 0x1000;

#[test]
fn the_byte_order_is_set_only_where_the_register_is() {
    for (size, set) in
        [(0, false), (4, false), (7, false), (8, true), (0x200, true), (0x1001, false)]
    {
        let bar = FakeBar::new(WINDOW);
        bar.present32(QEXT_SIZE as usize, size);
        assert_eq!(little_endian(Regs::new(bar.base()), WINDOW as u64), set, "size {size}");
        let wrote = bar.wrote32(QEXT_ENDIAN as usize) == QEXT_LITTLE_ENDIAN;
        assert_eq!(wrote, set, "size {size}");
    }
}

#[test]
fn a_mapping_that_ends_before_the_register_is_not_read_past() {
    /*
     * The size register says 8, but the broker mapped less than reaches the
     * byte-order register: the driver must neither read nor write past it.
     */
    let bar = FakeBar::new(WINDOW);
    bar.present32(QEXT_SIZE as usize, 8);
    let short = QEXT_ENDIAN as u64 + 3;
    assert!(!little_endian(Regs::new(bar.base()), short));
    assert_ne!(bar.wrote32(QEXT_ENDIAN as usize), QEXT_LITTLE_ENDIAN);
    assert!(little_endian(Regs::new(bar.base()), short + 1), "the whole register mapped");
}
