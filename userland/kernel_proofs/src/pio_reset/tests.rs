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

use super::reset_rule::resets_machine;

#[test]
fn every_command_that_drives_the_output_lines_is_refused() {
    for command in (0xF0u32..=0xFF).chain([0xD1]) {
        assert!(resets_machine(0x64, 1, command), "{command:#x}");
    }
}

#[test]
fn the_commands_a_ps2_driver_sends_go() {
    // Read and write the config byte, disable and enable each port, self
    // tests, and the write to the aux device.
    for command in [0x20u32, 0x60, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAD, 0xAE, 0xD0, 0xD2, 0xD3, 0xD4]
    {
        assert!(!resets_machine(0x64, 1, command), "{command:#x}");
    }
}

#[test]
fn data_written_to_0x60_is_not_a_command() {
    for value in [0xFEu32, 0xD1, 0xFF, 0xF4] {
        assert!(!resets_machine(0x60, 1, value), "{value:#x} to the data port");
    }
}

#[test]
fn a_wider_write_is_judged_by_the_byte_it_puts_at_0x64() {
    assert!(resets_machine(0x63, 2, 0xFE00), "the high byte lands on 0x64");
    assert!(!resets_machine(0x63, 2, 0x00FE), "the low byte lands on 0x63");
    assert!(resets_machine(0x61, 4, 0xD1_00_00_00), "the top byte lands on 0x64");
    assert!(!resets_machine(0x60, 4, 0xFE_FE_FE_FE), "0x60-0x63 only");
    assert!(resets_machine(0x64, 4, 0x0000_00FE));
}

#[test]
fn ports_past_the_command_port_are_not_judged() {
    for port in [0x65u16, 0x70, 0xFFFF] {
        assert!(!resets_machine(port, 4, 0xFEFE_FEFE));
    }
}
