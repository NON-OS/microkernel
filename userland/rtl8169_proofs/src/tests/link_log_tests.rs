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

//! The line for a half-duplex link on an 8168 (8-bit PHYstatus).

use nonos_libc::logged;

use super::memory::Memory;
use super::model::window;
use crate::constants::regs::REG_PHY_STATUS;
use crate::link::poll;

#[test]
fn a_half_duplex_100_link_on_an_8168_is_logged_as_such() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    bar.present8(REG_PHY_STATUS, 0x0A);
    assert!(poll(&mut d).up);
    assert_eq!(logged().last().cloned().unwrap_or_default(), "rtl8169: link up 100 half\n");
}
