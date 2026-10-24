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

//! The region rule's port and configuration halves, and the ranges it
//! refuses before asking anything.

use super::machine::pc;
use super::rule::{
    io, memory, pci_config, power_reg, power_touches, Hardware, Legacy, PowerReg, Refusal,
    SPACE_IO, SPACE_MEMORY,
};

#[test]
fn empty_and_wrapping_ranges_are_refused() {
    let m = pc();
    assert_eq!(memory(0x1000, 0, &m), Err(Refusal::Empty));
    assert_eq!(memory(u64::MAX - 1, 4, &m), Err(Refusal::Wraps));
    assert_eq!(io(0xFFFF, 2, &m), Err(Refusal::Wraps));
    assert_eq!(io(0x80, 0, &m), Err(Refusal::Empty));
}

#[test]
fn ports_firmware_uses_are_granted_and_legacy_ones_refused() {
    let m = pc();
    for (base, len) in
        [(0x62, 1), (0x66, 1), (0x72, 2), (0x80, 1), (0xB2, 2), (0x1800, 0x80), (0xFFFF, 1)]
    {
        assert_eq!(io(base, len, &m), Ok(()), "{base:#x}");
    }
    let legacy = [
        (0x00, Legacy::Dma),
        (0x21, Legacy::Pic),
        (0x43, Legacy::Pit),
        (0x81, Legacy::Dma),
        (0xA0, Legacy::Pic),
        (0xD0, Legacy::Dma),
        (0xCF9, Legacy::PciConfig),
        (0xCFC, Legacy::PciConfig),
        (0x70, Legacy::Rtc),
        (0x71, Legacy::Rtc),
        (0x3F8, Legacy::Serial),
        (0x3FF, Legacy::Serial),
    ];
    for (port, which) in legacy {
        assert_eq!(io(port, 1, &m), Err(Refusal::Legacy(which)), "{port:#x}");
    }
    assert_eq!(io(0x3E, 8, &m), Err(Refusal::Legacy(Legacy::Pit)), "a range reaching into the PIT");
    assert_eq!(io(0x60, 1, &m), Err(Refusal::Claimed), "the keyboard driver's port");
}

#[test]
fn pci_writes_to_a_claimed_function_are_refused_reads_are_not() {
    let m = pc();
    assert_eq!(pci_config((0, 0, 3, 0), false, &m), Ok(()));
    assert_eq!(pci_config((0, 0, 3, 0), true, &m), Err(Refusal::Claimed));
    assert_eq!(pci_config((0, 0, 0x1F, 0), true, &m), Ok(()));
}

#[test]
fn the_fadt_power_control_ports_are_refused_and_their_neighbours_are_not() {
    let m = pc();
    for (base, len) in [(0x604, 1), (0x605, 1), (0x600, 8), (0x5FF, 6)] {
        assert_eq!(
            io(base, len, &m),
            Err(Refusal::Legacy(Legacy::PowerControl)),
            "{base:#x}+{len}"
        );
    }
    for (base, len) in [(0x600, 4), (0x606, 2), (0x608, 4)] {
        assert_eq!(io(base, len, &m), Ok(()), "PM1 events and the timer stay AML's: {base:#x}");
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
