// NONOS Operating System (AGPL-3.0-or-later)

//! WiFi adapter discovery proofs: recognition is by PCI class, not brand, so a
//! wired NIC is excluded and an unknown wireless card is still listed.

use super::interface_rig::*;
use crate::interface::WifiInterface;
use crate::wifi_class::{discover, is_wifi, DeviceView, BUS_KIND_PCI, PCI_CLASS_NETWORK};

// WifiInterface is all-zero-valid (a byte array plus integer fields), so test
// buffers are sized with core::mem::zeroed and then filled by discover.

#[test]
fn recognises_wifi_by_class_not_brand() {
    // An unknown vendor's wireless card still counts as WiFi.
    assert!(is_wifi(&wifi(0x1234, 0x5678)));
    assert!(is_wifi(&wifi(0x8086, 0x51F0)));
    assert!(is_wifi(&wifi(0x10EC, 0xC821)));
}

#[test]
fn wired_ethernet_and_non_pci_are_not_wifi() {
    // Intel e1000: network class, but subclass 0x00 (Ethernet), not wireless.
    let eth = DeviceView {
        bus_kind: BUS_KIND_PCI,
        pci_class: PCI_CLASS_NETWORK,
        pci_subclass: 0x00,
        vendor: 0x8086,
        device: 0x100E,
    };
    assert!(!is_wifi(&eth));
    // Same ids on a non-PCI bus are not a PCI WiFi controller.
    let mut virt = wifi(0x8086, 0x51F0);
    virt.bus_kind = 2;
    assert!(!is_wifi(&virt));
    // A storage controller is not a network device at all.
    let mut storage = wifi(0x8086, 0x51F0);
    storage.pci_class = 0x01;
    assert!(!is_wifi(&storage));
}

#[test]
fn discover_lists_only_the_wifi_devices_and_names_them() {
    let devices = [
        wifi(0x8086, 0x51F0), // Intel AX210 CNVi
        DeviceView {
            // wired Ethernet, skipped
            bus_kind: BUS_KIND_PCI,
            pci_class: PCI_CLASS_NETWORK,
            pci_subclass: 0x00,
            vendor: 0x8086,
            device: 0x100E,
        },
        wifi(0x10EC, 0xC821), // Realtek RTL8821CE
        wifi(0x1234, 0x5678), // unknown wireless card
    ];
    let mut out: [WifiInterface; 8] = unsafe { core::mem::zeroed() };
    let n = discover(&devices, &mut out);
    assert_eq!(n, 3, "three WiFi adapters, the Ethernet NIC excluded");
    assert_eq!(out[0].name(), b"Intel Wi-Fi 6E AX210");
    assert_eq!(out[1].name(), b"Realtek RTL8821CE");
    assert_eq!(out[2].name(), b"Wi-Fi adapter", "unknown card still lists");
}
