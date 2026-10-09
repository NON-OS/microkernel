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

//! Discover the WiFi adapters present so the panel can list them before any
//! network scan. A WiFi controller is recognised by PCI class alone, not by
//! brand: it is a network-class device (0x02) with the "other network
//! controller" subclass (0x80), which is how wireless NICs are classed while
//! wired Ethernet uses subclass 0x00. Each adapter carries a friendly name
//! derived from its vendor and device id, and an unknown-but-valid card still
//! lists under a generic one. The panel only names the adapter: it neither
//! selects nor drives one, so the listed record keeps no device handle.

/// PCI base class for network controllers.
pub const PCI_CLASS_NETWORK: u8 = 0x02;
/// PCI subclass for "other" network controllers, which is where wireless NICs
/// report (Ethernet is subclass 0x00).
pub const PCI_SUBCLASS_WIRELESS: u8 = 0x80;
/// Bus kind tag for a PCI device, as `mk_device_list` reports it.
pub const BUS_KIND_PCI: u8 = 1;

/// The most an adapter name occupies, in bytes.
pub const NAME_MAX: usize = 32;

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

/// One discovered WiFi adapter as the panel lists it.
#[derive(Clone, Copy, Default)]
pub struct WifiInterface {
    name: [u8; NAME_MAX],
    name_len: usize,
    vendor: u16,
    device: u16,
}

impl WifiInterface {
    /// The friendly adapter name for display.
    pub fn name(&self) -> &[u8] {
        &self.name[..self.name_len]
    }

    /// The PCI vendor and device id.
    pub fn ids(&self) -> (u16, u16) {
        (self.vendor, self.device)
    }
}

/// Whether this build ships a driver that takes a Wi-Fi chip: the RTL8821CE
/// driver for 10ec:c821, and the iwlwifi driver for the Intel ids it matches
/// (`capsule_driver_iwlwifi/src/firmware/family.rs` `family_for_device`). The
/// kernel spawns iwlwifi for every Intel wireless function, but the driver
/// leaves at once on an id outside that list (0x24F7 to 0x24FA and 0x24FC are
/// no adapter's), so such a chip has no driver, not one that failed to start.
/// Anything else (MediaTek MT7921/7922, Broadcom, Qualcomm ath10k/ath11k,
/// other Realtek parts) has none either.
pub fn has_driver(vendor: u16, device: u16) -> bool {
    match vendor {
        0x10EC => device == 0xC821,
        0x8086 => matches!(
            device,
            0x08B1..=0x08B4
                | 0x095A
                | 0x095B
                | 0x3165
                | 0x3166
                | 0x24FB
                | 0x24F3..=0x24F6
                | 0x24FD
                | 0x2526
                | 0x9DF0
                | 0xA370
                | 0x31DC
                | 0x30DC
                | 0x271B
                | 0x271C
                | 0x2723
                | 0x34F0
                | 0x3DF0
                | 0x4DF0
                | 0x02F0
                | 0x06F0
                | 0x43F0
                | 0xA0F0
                | 0x2725
                | 0x2729
                | 0x51F0
                | 0x51F1
                | 0x54F0
                | 0xA74F
                | 0x272F
                | 0x7A70
                | 0x7AF0
                | 0x7F70
                | 0x7E40
        ),
        _ => false,
    }
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

fn interface_of(d: &DeviceView) -> WifiInterface {
    let mut name = [0u8; NAME_MAX];
    let label = adapter_label(d.vendor, d.device);
    let len = label.len().min(NAME_MAX);
    name[..len].copy_from_slice(&label.as_bytes()[..len]);
    WifiInterface { name, name_len: len, vendor: d.vendor, device: d.device }
}

/// A friendly name from vendor and device id. Known Intel and Realtek WiFi
/// parts get a specific label; any other WiFi-class card gets a generic vendor
/// label so it still lists.
pub fn adapter_label(vendor: u16, device: u16) -> &'static str {
    const INTEL: u16 = 0x8086;
    const REALTEK: u16 = 0x10EC;
    match (vendor, device) {
        (REALTEK, 0xC821) => "Realtek RTL8821CE",
        (INTEL, 0x2723 | 0x02F0 | 0x06F0 | 0x34F0 | 0x3DF0 | 0x4DF0 | 0x43F0) => {
            "Intel Wi-Fi 6 AX200"
        }
        (INTEL, 0x2725 | 0x2726 | 0x51F0 | 0x51F1 | 0x54F0 | 0x7AF0 | 0x7E40) => {
            "Intel Wi-Fi 6E AX210"
        }
        (INTEL, _) => "Intel Wi-Fi",
        (REALTEK, _) => "Realtek Wi-Fi",
        _ => "Wi-Fi adapter",
    }
}
