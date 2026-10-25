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

//! The FADT's power registers under the region rule: the reset register is
//! refused, PM1 control is the ACPI capsule's, and a register in memory is
//! protected whole.

use super::machine::pc;
use super::rule::{
    io, memory, power_reg, power_touches, Hardware, Legacy, PowerReg, Refusal, SPACE_IO,
    SPACE_MEMORY,
};

#[test]
fn the_reset_register_is_refused_and_pm1_control_is_granted() {
    let m = pc();
    for (base, len) in [(0x630, 1), (0x62F, 2), (0x62C, 8)] {
        assert_eq!(
            io(base, len, &m),
            Err(Refusal::Legacy(Legacy::ResetRegister)),
            "{base:#x}+{len}"
        );
    }
    for (base, len) in [(0x600, 4), (0x604, 2), (0x600, 8), (0x608, 4), (0x631, 1)] {
        assert_eq!(io(base, len, &m), Ok(()), "PM1 events, control and the timer: {base:#x}");
    }
}

#[test]
fn a_power_control_register_in_memory_is_protected() {
    let mut m = pc();
    m.hardware.push((0xFED8_0004, 0xFED8_0005, Hardware::PowerControl));
    assert_eq!(memory(0xFED8_0000, 0x10, &m), Err(Refusal::Protected(Hardware::PowerControl)));
}

#[test]
fn a_gas_becomes_the_bytes_its_bit_field_reaches() {
    let pm1 = power_reg(SPACE_IO, 0x604, 0, 16).expect("present");
    assert_eq!(pm1, PowerReg { space: SPACE_IO, start: 0x604, end: 0x606 });
    assert_eq!(power_reg(SPACE_IO, 0xCF9, 0, 8).map(|r| r.end), Some(0xCFA), "the reset register");
    assert_eq!(power_reg(SPACE_IO, 0x604, 4, 13).map(|r| r.end), Some(0x607), "offset and width");
    assert_eq!(
        power_reg(SPACE_MEMORY, 0xFED8_0004, 0, 0).map(|r| r.end),
        Some(0xFED8_0005),
        "no width"
    );
    assert_eq!(power_reg(SPACE_IO, 0, 0, 16), None, "absent");
    assert_eq!(power_reg(0x02, 0x604, 0, 16), None, "PCI config space is pci_config's");
    assert_eq!(power_reg(SPACE_MEMORY, u64::MAX, 0, 16), None, "wraps");
}

#[test]
fn touching_is_by_space_and_overlap() {
    let regs = [
        power_reg(SPACE_IO, 0x604, 0, 16).unwrap(),
        power_reg(SPACE_MEMORY, 0x1000, 0, 8).unwrap(),
    ];
    assert!(power_touches(regs.iter(), SPACE_IO, 0x605, 0x606));
    assert!(!power_touches(regs.iter(), SPACE_MEMORY, 0x604, 0x606), "another space");
    assert!(!power_touches(regs.iter(), SPACE_IO, 0x606, 0x700));
    assert!(power_touches(regs.iter(), SPACE_MEMORY, 0x0, 0x1001));
}
