// NONOS Operating System (AGPL-3.0-or-later)
//! Reset follows Linux's reboot=acpi order: ACPI, KBD, ACPI, KBD, CF9.

use crate::arch::x86_64::acpi::hw::fadt_decode::*;
use crate::arch::x86_64::acpi::hw::gas::Gas;
use crate::arch::x86_64::acpi::hw::reset::*;
use crate::bus_fake::{FakeBus, Op};
use crate::fadt_tests::{gemini_lake_fadt, FadtBuilder};

#[test]
fn the_sequence_is_acpi_kbd_acpi_kbd_cf9_with_delays() {
    let fadt = decode_fadt(&gemini_lake_fadt()).unwrap();
    let mut bus = FakeBus::default();
    bus.io.insert(0xCF9, 0xF1);
    reset_sequence(&mut bus, Some(&fadt));
    let w = bus.writes();
    let kbd = Op::IoWrite(0x64, 8, 0xFE);
    assert_eq!(w[0], Op::IoWrite(0xCF9, 8, 0x06), "ACPI reset register first");
    assert!(w[1..11].iter().all(|o| *o == kbd), "ten keyboard pulses");
    assert_eq!(w[11], Op::IoWrite(0xCF9, 8, 0x06), "ACPI again");
    assert!(w[12..22].iter().all(|o| *o == kbd));
    assert_eq!(w[22], Op::IoWrite(0xCF9, 8, 0xF1 & !0x0E | 0x02));
    assert_eq!(w[23], Op::IoWrite(0xCF9, 8, 0xF1 & !0x0E | 0x0E));
    assert_eq!(w.len(), 24);
    // 15 ms after each ACPI write, 50 us around each pulse and CF9 write.
    assert!(bus.ops.contains(&Op::Delay(15_000)));
    assert!(bus.delay_total() >= 2 * 15_000 + 20 * 100 + 100);
}

#[test]
fn without_reset_reg_sup_the_reset_register_is_not_touched() {
    let t = FadtBuilder::new(244)
        .gas(off::RESET_REG, Gas::io(0xCF9, 1))
        .u8(off::RESET_VALUE, 6)
        .sealed();
    let fadt = decode_fadt(&t).unwrap();
    let mut bus = FakeBus::default();
    assert!(!acpi_reset(&mut bus, &fadt));
    assert!(bus.ops.is_empty());
}

#[test]
fn a_memory_reset_register_goes_through_the_gas() {
    let t = FadtBuilder::new(244)
        .u32(off::FLAGS, FLAG_RESET_REG_SUP)
        .gas(
            off::RESET_REG,
            Gas { space: 0, bit_width: 8, bit_offset: 0, access_size: 1, address: 0xFED0_3004 },
        )
        .u8(off::RESET_VALUE, 0x0E)
        .sealed();
    let fadt = decode_fadt(&t).unwrap();
    let mut bus = FakeBus::default();
    assert!(acpi_reset(&mut bus, &fadt));
    assert_eq!(bus.writes(), [Op::MemWrite(0xFED0_3004, 8, 0x0E)]);
}

#[test]
fn a_pci_config_reset_register_is_a_byte_write_to_bus_0() {
    // Device 0x1F, function 0, offset 0x44 (an older ICH layout).
    let addr = (0x1Fu64 << 32) | 0x44;
    let t = FadtBuilder::new(244)
        .u32(off::FLAGS, FLAG_RESET_REG_SUP)
        .gas(
            off::RESET_REG,
            Gas { space: 2, bit_width: 8, bit_offset: 0, access_size: 1, address: addr },
        )
        .u8(off::RESET_VALUE, 0x06)
        .sealed();
    let fadt = decode_fadt(&t).unwrap();
    let mut bus = FakeBus::default();
    assert!(acpi_reset(&mut bus, &fadt));
    assert_eq!(bus.writes(), [Op::Pci(0x1F, 0, 0x44, 0x06)]);
    assert_eq!(pci_reset_target(32u64 << 32), None);
}

#[test]
fn an_io_reset_register_is_one_byte_whatever_its_width_says() {
    let t = FadtBuilder::new(244)
        .u32(off::FLAGS, FLAG_RESET_REG_SUP)
        .gas(
            off::RESET_REG,
            Gas { space: 1, bit_width: 32, bit_offset: 0, access_size: 3, address: 0xCF9 },
        )
        .u8(off::RESET_VALUE, 0x06)
        .sealed();
    let fadt = decode_fadt(&t).unwrap();
    let mut bus = FakeBus::default();
    acpi_reset(&mut bus, &fadt);
    assert_eq!(bus.writes(), [Op::IoWrite(0xCF9, 8, 0x06)]);
}

#[test]
fn the_keyboard_pulse_waits_for_the_input_buffer() {
    let mut bus = FakeBus::default();
    bus.io.insert(0x64, 0x02);
    kbd_reset(&mut bus);
    // The buffer never drains: each try polls 0x10000 times, then pulses.
    let polls = bus.ops.iter().filter(|o| **o == Op::IoRead(0x64, 8)).count();
    assert_eq!(polls, 10 * 0x10000);
    assert_eq!(bus.writes().len(), 10);
}
