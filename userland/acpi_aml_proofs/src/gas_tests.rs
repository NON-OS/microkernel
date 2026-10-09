// NONOS Operating System (AGPL-3.0-or-later)
//! GAS access widths and bit offsets follow ACPICA's acpi_hw_read/write.

use crate::arch::x86_64::acpi::hw::gas::*;
use crate::bus_fake::{FakeBus, Op};

fn io(addr: u64, width: u8, offset: u8, access: u8) -> Gas {
    Gas {
        space: SPACE_SYSTEM_IO,
        bit_width: width,
        bit_offset: offset,
        access_size: access,
        address: addr,
    }
}

#[test]
fn a_register_style_gas_uses_its_bit_width_and_ignores_access_size() {
    // PM1a_CNT as firmware often writes it: 16 bits wide, access size byte.
    let g = io(0x1804, 16, 0, 1);
    assert_eq!(g.access_bits(), 16);
    let mut bus = FakeBus::default();
    assert!(gas_write(&mut bus, &g, 0x3C01));
    assert_eq!(bus.ops, [Op::IoWrite(0x1804, 16, 0x3C01)]);
}

#[test]
fn port_io_is_never_wider_than_32_bits() {
    let g = io(0x400, 64, 0, 4);
    assert_eq!(g.access_bits(), 32);
    let mut bus = FakeBus::default();
    bus.io.insert(0x400, 0x1111_2222);
    bus.io.insert(0x404, 0x3333_4444);
    assert_eq!(gas_read(&mut bus, &g), Some(0x3333_4444_1111_2222));
}

#[test]
fn access_size_decides_when_the_gas_is_not_register_style() {
    // 4 bits at offset 2, byte access: one byte read, raw value returned.
    let g = io(0xB2, 4, 2, 1);
    assert_eq!(g.access_bits(), 8);
    let mut bus = FakeBus::default();
    bus.io.insert(0xB2, 0xAC);
    assert_eq!(gas_read(&mut bus, &g), Some(0xAC));
}

#[test]
fn whole_units_below_the_bit_offset_are_skipped() {
    // 8-bit field at bit offset 8, byte access: first byte not touched.
    let g = io(0x60, 8, 8, 1);
    let mut bus = FakeBus::default();
    assert!(gas_write(&mut bus, &g, 0xAB00));
    assert_eq!(bus.ops, [Op::IoWrite(0x61, 8, 0xAB)]);
}

#[test]
fn a_zero_access_size_falls_back_to_an_aligned_power_of_two() {
    let g = Gas {
        space: SPACE_SYSTEM_MEMORY,
        bit_width: 24,
        bit_offset: 0,
        access_size: 0,
        address: 0x1002,
    };
    // 24 bits rounds to 32, narrowed to 16 by the 2-byte alignment.
    assert_eq!(g.access_bits(), 16);
}

#[test]
fn memory_registers_are_written_through_memory_cycles() {
    let g = Gas {
        space: SPACE_SYSTEM_MEMORY,
        bit_width: 32,
        bit_offset: 0,
        access_size: 3,
        address: 0xFE00_0004,
    };
    let mut bus = FakeBus::default();
    assert!(gas_write(&mut bus, &g, 0x2000));
    assert_eq!(bus.ops, [Op::MemWrite(0xFE00_0004, 32, 0x2000)]);
}

#[test]
fn absent_unknown_or_out_of_range_registers_are_refused() {
    let mut bus = FakeBus::default();
    assert_eq!(gas_read(&mut bus, &Gas::empty()), None);
    let ec = Gas { space: 3, bit_width: 8, bit_offset: 0, access_size: 1, address: 0x10 };
    assert!(!gas_write(&mut bus, &ec, 1));
    let past_ports = io(0xFFFF, 16, 0, 2);
    assert!(!gas_write(&mut bus, &past_ports, 1));
    let too_wide = io(0x400, 64, 8, 0);
    assert_eq!(gas_read(&mut bus, &too_wide), None);
    assert!(bus.ops.is_empty());
}

#[test]
fn a_firmware_gas_decodes_from_its_twelve_bytes() {
    let b = [1u8, 16, 0, 2, 0x04, 0x18, 0, 0, 0, 0, 0, 0];
    assert_eq!(Gas::from_bytes(&b), io(0x1804, 16, 0, 2));
}
