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

//! The rows a hypervisor's bus and the ACPI capsule take: the VMBus root and
//! the SCSI controller's channel by their ids on BUS_KIND_VIRT, and the SCI
//! record by its PnP id, nothing more.

use super::scope::{admits, Seen};

fn seen(bus: u8, vendor: u16, device: u16, class: u32) -> Seen {
    Seen { bus, vendor, device, class, pci_class: 0, pci_sub: 0, progif: 0 }
}

#[test]
fn the_virt_bus_kind_is_the_brokers() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/hardware/broker");
    let bus = std::fs::read_to_string(root.join("device/bus.rs")).expect("readable");
    let rule = std::fs::read_to_string(root.join("scope_match.rs")).expect("readable");
    assert!(bus.contains("const BUS_KIND_VIRT: u8 = 3;") && rule.contains("const VIRT: u8 = 3;"));
}

#[test]
fn only_vmbus0_claims_the_vmbus_root_and_only_on_the_virt_bus() {
    let root = seen(3, 0x1414, 0x5642, 0xFFFF);
    assert!(admits("driver.vmbus0", &root));
    for s in ["driver.virtio_net0", "driver.vmxnet3_0", "acpi", "driver.hda0"] {
        assert!(!admits(s, &root), "{s}");
    }
    for bus in [1, 2] {
        assert!(!admits("driver.vmbus0", &seen(bus, 0x1414, 0x5642, 0xFFFF)), "bus {bus}");
    }
    assert!(!admits("driver.vmbus0", &seen(3, 0x1414, 0x5643, 0xFFFF)));
}

#[test]
fn only_storvsc0_claims_the_scsi_channel_and_only_on_the_virt_bus() {
    let scsi = seen(3, 0x1414, 0x5343, 0xFFFF);
    assert!(admits("driver.storvsc0", &scsi));
    for s in ["driver.vmbus0", "driver.pvscsi0", "driver.virtio_scsi0", "acpi"] {
        assert!(!admits(s, &scsi), "{s}");
    }
    assert!(!admits("driver.storvsc0", &seen(1, 0x1414, 0x5343, 0xFFFF)), "a PCI function");
    for device in [0x5642, 0x4E45, 0x4944] {
        assert!(!admits("driver.storvsc0", &seen(3, 0x1414, device, 0xFFFF)), "{device:#x}");
    }
}

#[test]
fn the_acpi_capsule_claims_the_sci_record_alone() {
    assert!(admits("acpi", &seen(2, 0x0001, 0x0C08, 0xFFFF)));
    assert!(!admits("acpi", &seen(2, 0x0001, 0x0303, 0x0040)), "a PS/2 port");
    assert!(!admits("acpi", &seen(2, 0x0001, 0x0C09, 0xFFFF)));
    assert!(!admits("acpi", &seen(1, 0x0001, 0x0C08, 0xFFFF)), "a PCI function");
    assert!(!admits("driver.ps2_kbd0", &seen(2, 0x0001, 0x0C08, 0xFFFF)));
}

#[test]
fn both_svga_layouts_and_no_other_display() {
    assert!(admits("driver.vmware_svga0", &seen(1, 0x15AD, 0x0405, 0)));
    assert!(admits("driver.vmware_svga0", &seen(1, 0x80EE, 0xBEEF, 0)));
    assert!(!admits("driver.vmware_svga0", &seen(1, 0x1234, 0x1111, 0)), "bga's");
    assert!(!admits("driver.vmware_svga0", &seen(1, 0x1AF4, 0x1050, 0)), "virtio-gpu's");
}
