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

use super::rule::{after_assign, while_sizing, IO_SPACE, MEMORY_SPACE};

const DECODERS: u16 = IO_SPACE | MEMORY_SPACE;
/// The Command register's Bus Master Enable, bit 2.
const BUS_MASTER: u16 = 1 << 2;

#[test]
fn the_boot_pass_never_turns_bus_mastering_on() {
    // Every value a Command register can hold: Bus Master leaves as it came.
    for found in 0..=u16::MAX {
        assert_eq!(after_assign(found) & BUS_MASTER, found & BUS_MASTER, "found {found:#06x}");
        assert_eq!(while_sizing(found) & BUS_MASTER, found & BUS_MASTER, "found {found:#06x}");
    }
}

#[test]
fn the_decoders_are_off_while_sizing_and_on_after() {
    for found in [0u16, DECODERS, BUS_MASTER, 0x0407, 0xFFFF] {
        assert_eq!(while_sizing(found) & DECODERS, 0);
        assert_eq!(after_assign(found) & DECODERS, DECODERS);
        // Nothing else the register held is touched.
        assert_eq!(after_assign(found) & !DECODERS, found & !DECODERS);
    }
}
