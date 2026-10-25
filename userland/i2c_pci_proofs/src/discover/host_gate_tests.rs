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


//! The kernel starts driver.i2c_pci only where inventory::i2c_host takes a
//! record the broker lists. That rule may be wider than the capsule's own
//! match, never narrower: a record the capsule would drive always starts it.

use nonos_libc::{DeviceRecord, BUS_KIND_ACPI, BUS_KIND_PCI, BUS_KIND_VIRT};

use super::{classify::classify, kernel_i2c_host::i2c_host};

fn record(bus: u8, vendor: u16, device: u16, class: u32, pci: (u8, u8)) -> DeviceRecord {
    let mut r = DeviceRecord::empty();
    (r.bus_kind, r.vendor, r.device, r.class) = (bus, vendor, device, class);
    (r.pci_class, r.pci_subclass) = pci;
    r
}

fn starts(r: &DeviceRecord) -> bool {
    i2c_host(r.bus_kind, r.vendor, r.device, r.class, (r.pci_class, r.pci_subclass))
}

const CLASSES: [(u8, u8); 6] = [(0, 0), (0x0C, 0x80), (0x11, 0x80), (12, 5), (12, 0), (17, 0)];

#[test]
fn every_record_the_capsule_takes_starts_it() {
    for pci in CLASSES {
        for device in 0..=u16::MAX {
            let r = record(BUS_KIND_PCI, 0x8086, device, 0, pci);
            assert!(classify(&r).is_none() || starts(&r), "8086:{device:04x} {pci:?}");
        }
        for bus in [BUS_KIND_PCI, BUS_KIND_ACPI, BUS_KIND_VIRT] {
            for vendor in [0x0000, 0x1022, 0x1AF4, 0x1B36, 0x8086] {
                for class in [0, 0x0041, 0x0060] {
                    let r = record(bus, vendor, 0x9D60, class, pci);
                    assert!(classify(&r).is_none() || starts(&r), "{bus} {vendor:x} {class:x}");
                }
            }
        }
    }
}

#[test]
fn qemus_q35_lists_none() {
    let q35 = [
        (0x8086, 0x29C0, (0x06, 0x00)), (0x8086, 0x2918, (0x06, 0x01)), (0x8086, 0x2922, (1, 6)),
        (0x8086, 0x2930, (0x0C, 0x05)), (0x8086, 0x293E, (0x04, 0x03)), (0x1B36, 0x0D, (0x0C, 3)),
        (0x1B36, 0x0010, (0x01, 0x08)), (0x1AF4, 0x1041, (0x02, 0x00)),
    ];
    for (vendor, device, pci) in q35 {
        assert!(!starts(&record(BUS_KIND_PCI, vendor, device, 0, pci)), "{vendor:x}:{device:x}");
    }
}

#[test]
fn an_lpss_function_and_an_acpi_host_do() {
    assert!(starts(&record(BUS_KIND_PCI, 0x8086, 0x9D60, 0, (0x11, 0x80))));
    assert!(starts(&record(BUS_KIND_PCI, 0x8086, 0x7E78, 0, (0x0C, 0x80))));
    assert!(starts(&record(BUS_KIND_ACPI, 0, 0, 0x0060, (0x0C, 0))));
    assert!(!starts(&record(BUS_KIND_ACPI, 0, 0, 0x0041, (0, 0))));
}
