// NONOS Operating System (AGPL-3.0-or-later)
//! WiFi adapter discovery proofs: recognition is by PCI class, not brand, so a
//! wired NIC is excluded and an unknown wireless card is still listed. The
//! friendly names for the parts we target (Realtek RTL8821CE, Intel AX200/AX210
//! including the Alder Lake CNVi 0x51f0) are pinned.

use crate::interface::{
    adapter_label, discover, has_driver, is_wifi, DeviceView, WifiInterface, BUS_KIND_PCI,
    PCI_CLASS_NETWORK, PCI_SUBCLASS_WIRELESS,
};

fn wifi(vendor: u16, device: u16) -> DeviceView {
    DeviceView {
        bus_kind: BUS_KIND_PCI,
        pci_class: PCI_CLASS_NETWORK,
        pci_subclass: PCI_SUBCLASS_WIRELESS,
        vendor,
        device,
    }
}

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

#[test]
fn discover_never_overruns_the_output_buffer() {
    let devices = [wifi(0x10EC, 0xC821), wifi(0x8086, 0x51F0), wifi(0x1234, 0x5678)];
    let mut out: [WifiInterface; 2] = unsafe { core::mem::zeroed() };
    let n = discover(&devices, &mut out);
    assert_eq!(n, 2, "fills to capacity and drops the rest");
    assert_eq!(out[0].name(), b"Realtek RTL8821CE");
    assert_eq!(out[1].name(), b"Intel Wi-Fi 6E AX210");
}

#[test]
fn known_parts_get_specific_names() {
    assert_eq!(adapter_label(0x10EC, 0xC821), "Realtek RTL8821CE");
    assert_eq!(adapter_label(0x8086, 0x51F0), "Intel Wi-Fi 6E AX210");
    assert_eq!(adapter_label(0x8086, 0x2723), "Intel Wi-Fi 6 AX200");
    assert_eq!(adapter_label(0x8086, 0x1234), "Intel Wi-Fi");
    assert_eq!(adapter_label(0x10EC, 0x9999), "Realtek Wi-Fi");
    assert_eq!(adapter_label(0x1111, 0x2222), "Wi-Fi adapter");
}

/* Settings says a chip with no NONOS driver plainly instead of waiting on a
driver that will never answer: the inventory spawns iwlwifi for Intel wireless
and the RTL8821CE driver for 10ec:c821, and nothing else. */
#[test]
fn only_the_chips_a_driver_is_spawned_for_have_one() {
    assert!(has_driver(0x10EC, 0xC821));
    for intel in [0x095A, 0x24FD, 0x2526, 0x9DF0, 0x2723, 0x06F0, 0x2725, 0x51F0, 0x7AF0, 0x7F70] {
        assert!(has_driver(0x8086, intel), "{intel:#x}");
    }
    // The 3165 and Tiger Lake's AX201 are found by the driver (b614f22a).
    for intel in [0xA0F0, 0x3165, 0x3166] {
        assert!(has_driver(0x8086, intel), "{intel:#x}");
    }
    // Intel ids the iwlwifi driver leaves at once on: no adapter's. The chip
    // has no driver; it is not one that failed.
    for intel in [0x24F7, 0x24F8, 0x24F9, 0x24FA, 0x24FC] {
        assert!(!has_driver(0x8086, intel), "{intel:#x}");
    }
    // MediaTek MT7921/MT7922, Broadcom, Qualcomm ath10k/ath11k, RTL8822CE/BE.
    for (v, d) in [
        (0x14C3, 0x7961),
        (0x14C3, 0x0616),
        (0x14E4, 0x43A0),
        (0x168C, 0x003E),
        (0x17CB, 0x1103),
        (0x10EC, 0xC822),
        (0x10EC, 0xB822),
    ] {
        assert!(!has_driver(v, d), "{v:#x}:{d:#x}");
    }
}

#[test]
fn a_discovered_adapter_keeps_its_ids() {
    let devices = [wifi(0x14C3, 0x0616)];
    let mut out: [WifiInterface; 1] = unsafe { core::mem::zeroed() };
    assert_eq!(discover(&devices, &mut out), 1);
    assert_eq!(out[0].ids(), (0x14C3, 0x0616));
}

/* The panel's Intel rule is the iwlwifi driver's own id table, every id: a
chip the driver takes has a driver, one it leaves at once on does not. */
#[test]
fn the_intel_rule_is_the_drivers_own_id_table() {
    for id in 0..=u16::MAX {
        assert_eq!(
            has_driver(0x8086, id),
            crate::iwlwifi_family::family_for_device(id).is_some(),
            "8086:{id:04x}"
        );
    }
}
