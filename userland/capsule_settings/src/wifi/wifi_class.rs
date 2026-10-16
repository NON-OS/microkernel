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

//! A Wi-Fi controller known by its PCI class alone, not its brand: a
//! network-class device (0x02) with the "other network controller"
//! subclass (0x80), as wireless NICs are classed while wired Ethernet uses
//! 0x00; and the adapters found in a device list.

use super::interface::{interface_of, WifiInterface};

/// PCI base class for network controllers.
pub const PCI_CLASS_NETWORK: u8 = 0x02;

/// PCI subclass for "other" network controllers, which is where wireless NICs
/// report (Ethernet is subclass 0x00).
pub const PCI_SUBCLASS_WIRELESS: u8 = 0x80;

/// Bus kind tag for a PCI device, as `mk_device_list` reports it.
pub const BUS_KIND_PCI: u8 = 1;

/// The minimal device facts the panel needs to recognise and list an adapter.
/// This mirrors the fields of the broker `DeviceRecord` the panel reads, kept
/// separate so the discovery logic stays pure and host-testable.
#[derive(Clone, Copy)]
pub struct DeviceView {
    pub bus_kind: u8,
    pub pci_class: u8,
    pub pci_subclass: u8,
    pub vendor: u16,
    pub device: u16,
}

/// True when a device is a WiFi controller, judged by PCI class alone.
pub fn is_wifi(d: &DeviceView) -> bool {
    d.bus_kind == BUS_KIND_PCI
        && d.pci_class == PCI_CLASS_NETWORK
        && d.pci_subclass == PCI_SUBCLASS_WIRELESS
}

/// Fill `out` with the WiFi adapters found in `devices`, returning how many
/// were written. Extra devices past `out`'s capacity are dropped rather than
/// overrun.
pub fn discover(devices: &[DeviceView], out: &mut [WifiInterface]) -> usize {
    let mut n = 0;
    for d in devices {
        if n == out.len() {
            break;
        }
        if is_wifi(d) {
            out[n] = interface_of(d);
            n += 1;
        }
    }
    n
}
