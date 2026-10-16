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

use super::slots::{bar_slots, upper_half_fits};

// The dword after a header's last BAR, where its bridge registers begin.
fn after_bars(header: u8) -> u16 {
    0x10 + 4 * bar_slots(header) as u16
}

#[test]
fn each_header_type_has_its_own_bars_and_an_unknown_one_none() {
    for multi in [0x00, 0x80] {
        assert_eq!(bar_slots(multi), 6);
        assert_eq!(bar_slots(multi | 0x01), 2);
        assert_eq!(bar_slots(multi | 0x02), 1);
        for unknown in 0x03..=0x7F {
            assert_eq!(bar_slots(multi | unknown), 0, "{unknown:#x}");
        }
    }
    // A function that is not there reads all-ones: nothing probed.
    assert_eq!(bar_slots(0xFF), 0);
}

#[test]
fn a_bridges_bus_numbers_and_a_cardbus_sockets_registers_are_never_reached() {
    assert_eq!(after_bars(0x01), 0x18, "a bridge's primary, secondary and subordinate bus");
    assert_eq!(after_bars(0x02), 0x14, "a CardBus bridge's capabilities and status");
    assert_eq!(after_bars(0x00), 0x28, "a type 0 header's CardBus CIS pointer");
}

#[test]
fn no_upper_half_is_read_past_the_last_bar() {
    for header in 0..=u8::MAX {
        let slots = bar_slots(header);
        for index in 0..8u8 {
            let reached = 0x10 + 4 * (index as u16 + 1);
            if index < slots && upper_half_fits(index, slots) {
                assert!(reached < after_bars(header), "{header:#x} {index}");
            }
        }
    }
    assert!(upper_half_fits(4, 6) && !upper_half_fits(5, 6));
    assert!(upper_half_fits(0, 2) && !upper_half_fits(1, 2) && !upper_half_fits(0, 1));
    assert!(!upper_half_fits(0, 0) && !upper_half_fits(255, 255));
}
