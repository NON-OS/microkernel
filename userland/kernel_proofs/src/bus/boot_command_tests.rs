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

//! the boot pass turns decoders on and never sets Bus Master; it
//! clears it on every function but a display, which keeps it as found.

use super::boot_command::{boot_command, BUS_MASTER, CLASS_DISPLAY, IO_SPACE, MEMORY_SPACE};

#[test]
fn bus_master_is_never_set_and_off_but_on_a_display() {
    for found in 0..=u16::MAX {
        for class in 0..=0xFFu8 {
            let left = boot_command(found, class);
            assert_eq!(left & (IO_SPACE | MEMORY_SPACE), IO_SPACE | MEMORY_SPACE);
            assert!(left & BUS_MASTER == 0 || found & BUS_MASTER != 0, "set where it was off");
            if class != CLASS_DISPLAY {
                assert_eq!(left & BUS_MASTER, 0, "{found:#x} class {class:#x}");
            }
            assert_eq!(
                left & !(IO_SPACE | MEMORY_SPACE | BUS_MASTER),
                found & !(IO_SPACE | MEMORY_SPACE | BUS_MASTER)
            );
        }
    }
}

#[test]
fn a_display_keeps_what_firmware_left() {
    assert_eq!(boot_command(BUS_MASTER, CLASS_DISPLAY) & BUS_MASTER, BUS_MASTER);
    assert_eq!(boot_command(0, CLASS_DISPLAY) & BUS_MASTER, 0);
    assert_eq!(boot_command(0x0007, 0x02), 0x0003, "a NIC firmware left mastering");
}
