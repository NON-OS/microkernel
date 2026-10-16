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

//! The register accessors are safe to call: an offset outside the window
//! reaches nothing. A memory window answers all ones past its end and drops
//! a write; a port offset past sixteen bits reaches no port (with no
//! device attached here, one that did would panic).

use nonos_devmodel::FakeBar;

use super::model::view;
use crate::regs::Regs;

#[test]
fn an_access_outside_the_memory_window_reaches_nothing() {
    let bar = FakeBar::new(32);
    bar.present32(28, 0x1122_3344);
    let regs = Regs::mmio(view(&bar));
    assert_eq!((regs.r8(32), regs.r16(31), regs.r32(30)), (0xFF, 0xFFFF, u32::MAX));
    assert_eq!(regs.r64(28), 0xFFFF_FFFF_1122_3344, "the high half lies past the end");
    assert_eq!(regs.r64(usize::MAX), u64::MAX, "an offset that would wrap");
    regs.w32(32, 7);
    regs.w8(usize::MAX, 7);
    assert!((0..28).all(|at| bar.wrote8(at) == 0), "a write outside landed inside");
    assert_eq!(bar.wrote32(28), 0x1122_3344);
}

#[test]
fn a_port_offset_past_sixteen_bits_reaches_no_port() {
    let regs = Regs::pio(5);
    for off in [0x1_0000usize, usize::MAX] {
        assert_eq!((regs.r8(off), regs.r16(off), regs.r32(off)), (0, 0, 0));
        regs.w8(off, 1);
        regs.w16(off, 1);
        regs.w32(off, 1);
    }
}
