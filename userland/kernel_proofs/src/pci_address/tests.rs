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

use super::address_packing::pci_config_address;
use super::device::DeviceAddress;
use super::types::PciDevice;

fn pci(bus: u8, device: u8, function: u8) -> PciDevice {
    PciDevice {
        segment: 0,
        bus,
        device,
        function,
        vendor_id: 0,
        device_id: 0,
        class: 0,
        subclass: 0,
    }
}

#[test]
fn device_address_never_leaves_its_bus() {
    for device in 0..=u8::MAX {
        for function in 0..=u8::MAX {
            let a = DeviceAddress::pci(0, device, function);
            assert_eq!(a.pci_bus(), 0, "device {device} function {function}");
            assert_eq!(a.pci_device(), device & 0x1F);
            assert_eq!(a.pci_function(), function & 0x7);
        }
    }
    assert_ne!(DeviceAddress::pci(0, 32, 0), DeviceAddress::pci(1, 0, 0));
}

#[test]
fn bdf_high_byte_is_the_bus() {
    for device in 0..=u8::MAX {
        for function in 0..=u8::MAX {
            let v = pci(7, device, function).bdf();
            assert_eq!(v >> 8, 7, "device {device} function {function}");
            assert_eq!((v >> 3) & 0x1F, u16::from(device & 0x1F));
            assert_eq!(v & 0x7, u16::from(function & 0x7));
        }
    }
    assert_ne!(pci(0, 0, 8).bdf(), pci(0, 1, 0).bdf());
}

#[test]
fn config_address_bus_field_is_the_bus() {
    for device in 0..=u8::MAX {
        for function in 0..=u8::MAX {
            let word = pci_config_address(3, device, function, 0x40);
            assert_eq!((word >> 16) & 0xFF, 3, "device {device} function {function}");
            assert_eq!((word >> 11) & 0x1F, u32::from(device & 0x1F));
            assert_eq!((word >> 8) & 0x7, u32::from(function & 0x7));
        }
    }
    assert_ne!(pci_config_address(0, 32, 0, 0), pci_config_address(1, 0, 0, 0));
}
