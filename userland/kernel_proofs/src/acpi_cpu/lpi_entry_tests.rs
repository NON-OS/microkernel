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

//! A state's entry must be a register or an integer; its counters a
//! register or absent.

use super::build::reg;
use super::cpu_layout::*;
use super::cpu_wire::decode;
use super::lpi_build::{level, state};

#[test]
fn entries_and_counters_are_checked() {
    let one = |entry: Vec<u8>| {
        let mut b = super::build::levels_header(0, 0, 0, 1, 0, 0);
        b.extend(level(LPI_OWN, 0, 0, 1));
        b.extend(state(1, 0, entry));
        b
    };
    assert!(decode(&one(reg(REGISTER, 0x7F, 32, 1))).is_some());
    assert!(decode(&one(reg(INTEGER, 0, 0, 1))).is_some());
    assert_eq!(decode(&one(reg(ABSENT, 0, 0, 0))), None, "a state must be enterable");
    assert_eq!(decode(&one(reg(REGISTER, 0x02, 32, 0))), None, "PCI config space");
    let mut usage = one(reg(INTEGER, 0, 0, 1));
    let at = usage.len() - REG;
    usage[at] = INTEGER;
    assert_eq!(decode(&usage), None, "a counter is a register or absent");
}
