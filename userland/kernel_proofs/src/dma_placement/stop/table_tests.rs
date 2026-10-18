// NONOS Operating System (AGPL-3.0-or-later)
//! Which devices MkDmaMap may hand frames, over every bus kind and class.

use super::acpi_stop::{acpi_stop, may_map, Stop, BLOCK, BUS_ACPI};

/// From the crate, where tests run.
const SRC: &str = "../../src/hardware/broker";

/// The class ids the device table holds (`class::ids`), by value.
const CLASSES: [u32; 12] =
    [0x01, 0x10, 0x20, 0x30, 0x40, 0x41, 0x50, 0x60, 0x70, 0x71, 0x80, 0xFFFF];

#[test]
fn only_an_acpi_sd_host_of_the_acpi_kinds_gets_frames() {
    let mut refused = 0;
    let classes = CLASSES.into_iter().chain(0..0x200).chain([u32::MAX]);
    for (bus, class) in classes.flat_map(|c| (0..=8u8).map(move |b| (b, c))) {
        let want = bus != 2 || class == 0x10;
        assert_eq!(may_map(bus, class), want, "bus {bus} class {class:#x}");
        refused += usize::from(!want);
    }
    // The kinds the platform rule records: SERIAL, BLOCK, GPIO_CTRL,
    // INPUT, I2C_HID, OTHER.
    for (class, stop) in [(0x60, Stop::NoDma), (0x10, Stop::SdhciResetAll), (0x80, Stop::NoDma)]
        .into_iter()
        .chain([(0x40, Stop::NoDma), (0x41, Stop::NoDma), (0xFFFF, Stop::NoDma)])
    {
        assert_eq!(acpi_stop(class), stop, "{class:#x}");
    }
    println!("{refused} (bus, class) pairs refused");
    assert!(refused >= 420, "{refused}");
}

/// The rule restates two numbers so it can stay pure; they must be the
/// device table's own.
#[test]
fn the_restated_numbers_are_the_tables() {
    let bus = std::fs::read_to_string(format!("{SRC}/device/bus.rs")).unwrap();
    assert!(bus.contains(&format!("const BUS_KIND_ACPI: u8 = {BUS_ACPI};")));
    let class = std::fs::read_to_string(format!("{SRC}/class.rs")).unwrap();
    assert!(class.contains(&format!("const BLOCK: u32 = {BLOCK:#06x};")));
}
