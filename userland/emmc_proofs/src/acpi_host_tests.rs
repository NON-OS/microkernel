// NONOS Operating System (AGPL-3.0-or-later)
//! Which broker records are SD hosts to try: a PCI function of the class,
//! and an ACPI platform device as F 28 publishes one (class 08h/05h/00h,
//! vendor and device 0), never one the broker marks disabled, never one of
//! another bus or class.

use crate::emmc::pci::{host, Kind};

const PCI: u8 = 1;
const ACPI: u8 = 2;
const VIRT: u8 = 3;
const DISABLED: u32 = 1 << 1;
const SD: (u8, u8, u8) = (0x08, 0x05, 0x00);

#[test]
fn an_acpi_sd_host_is_tried_as_a_generic_acpi_host() {
    assert_eq!(host((ACPI, 0), 0, 0, SD), Some((Kind::Generic, true)));
    assert_eq!(host((PCI, 0), 0x8086, 0x31cc, (0x08, 0x05, 0x01)), Some((Kind::IntelEmmc, false)));
}

#[test]
fn a_disabled_record_another_bus_or_another_class_is_not_tried() {
    assert_eq!(host((ACPI, DISABLED), 0, 0, SD), None);
    assert_eq!(host((PCI, DISABLED), 0x1217, 0x8620, SD), None);
    assert_eq!(host((VIRT, 0), 0, 0, SD), None);
    assert_eq!(host((ACPI, 0), 0, 0, (0x0c, 0x80, 0x00)), None, "an I2C host's record");
    assert_eq!(host((ACPI, 0), 0, 0, (0x08, 0x05, 0x03)), None);
}
