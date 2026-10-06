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

//! The RTL8153B (RTL_VER_08 and 09) through its r8153b steps.

use crate::chip::{bound, PHY, PLA, USB};
use crate::r8153::Version;

#[test]
fn an_rtl8153b_takes_its_own_steps() {
    let (nic, _, chip) = bound(0x6010);
    let r = &chip.borrow().regs;
    assert_eq!(nic.version(), Version::V09);
    assert_eq!(r.byte(PLA, 0xc0bc), 0xff, "Teredo bits written one to clear");
    assert_eq!(r.dword(USB, 0xd40c), 0x0001_0001, "RX_THR_B");
    assert_eq!(r.word(PHY, 0xa436), 0, "no RTL8153 SRAM tuning");
    assert_eq!(r.word(PHY, 0xbc06), 0, "no ADC config");
}
