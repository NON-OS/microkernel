// NONOS Operating System (AGPL-3.0-or-later)
//! Discovery finds every xHCI however many other functions the machine
//! lists ahead of them, the chipset's first.

use super::find::find_all_xhci;
use crate::constants::CLASS_USB_HOST_XHCI;
use nonos_libc::{offer_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

fn record(id: u64, class: u32, pci: [u8; 3], device: u16) -> DeviceRecord {
    let mut r = DeviceRecord::empty();
    (r.device_id, r.class, r.bus_kind, r.vendor, r.device) =
        (id, class, BUS_KIND_PCI, 0x8086, device);
    (r.pci_class, r.pci_subclass, r.pci_progif) = (pci[0], pci[1], pci[2]);
    r.bar_count = 1;
    r.bars[0] = Bar { size: 0x1_0000, kind: BAR_KIND_MMIO, ..Bar::empty() };
    r
}

#[test]
fn a_controller_past_the_hundredth_function_is_found() {
    // A big machine: 100 bridges and others, then Thunderbolt's xHCI,
    // then the chipset's (Meteor Lake's TCSS and PCH ids).
    let mut machine: Vec<DeviceRecord> =
        (0..100).map(|i| record(i, 0x0010, [0x06, 0x04, 0x00], 0x7ec4)).collect();
    machine.push(record(200, CLASS_USB_HOST_XHCI, [0x0c, 0x03, 0x30], 0x7ec0));
    machine.push(record(201, CLASS_USB_HOST_XHCI, [0x0c, 0x03, 0x30], 0x7e7d));
    offer_devices(machine);
    let found = find_all_xhci();
    let ids: Vec<(u64, u16)> = found.iter().map(|f| (f.device_id, f.device)).collect();
    assert_eq!(ids, vec![(201, 0x7e7d), (200, 0x7ec0)], "both, the chipset's first");
}

#[test]
fn an_ehci_in_the_xhci_class_is_not_taken() {
    offer_devices(vec![record(1, CLASS_USB_HOST_XHCI, [0x0c, 0x03, 0x20], 0x7e7d)]);
    assert!(find_all_xhci().is_empty());
}
