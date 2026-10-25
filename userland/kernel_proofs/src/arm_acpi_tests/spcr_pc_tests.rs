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

//! SPCR on a PC server, through the same parser: a BMC's 16550 in I/O
//! space, named by GAS (interface 0x12).

use super::build::*;
use super::spcr_tests::table;
use crate::arm_acpi::spcr::{spcr, Interface};

#[test]
fn a_pc_servers_spcr_by_gas() {
    let mut t = table(2, false, b"");
    t[36] = 0x12;
    t[40] = 1;
    t[44..52].copy_from_slice(&u64le(0x3F8));
    seal(&mut t);
    let s = spcr(&t).unwrap();
    assert_eq!((s.interface, s.base.space, s.base.address), (Interface::Ns16550Gas, 1, 0x3F8));
}
