// NONOS Operating System (AGPL-3.0-or-later)
//! The ids the driver takes are rtw88's for the 8822CE (rtw8822ce.c), and
//! discovery takes only them, with their first sized memory BAR.

use nonos_libc::{DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::{PCI_VENDOR_REALTEK, RTL8822CE_DEVICE_IDS};
use crate::discover::pick;

#[test]
fn the_ids_are_rtw88s() {
    assert_eq!(PCI_VENDOR_REALTEK, 0x10EC);
    assert_eq!(RTL8822CE_DEVICE_IDS, [0xC822, 0xC82F]);
}

fn record(vendor: u16, device: u16) -> DeviceRecord {
    let mut r = DeviceRecord::empty();
    r.bus_kind = BUS_KIND_PCI;
    r.vendor = vendor;
    r.device = device;
    r.device_id = 7;
    r.bar_count = 3;
    r.bars[2].kind = BAR_KIND_MMIO;
    r.bars[2].size = 0x10000;
    r
}

#[test]
fn both_ids_are_taken_with_their_memory_bar() {
    for id in RTL8822CE_DEVICE_IDS {
        let f = pick(&[record(0x10EC, id)]).expect("taken");
        assert_eq!((f.device_id, f.bar_index, f.bar_size), (7, 2, 0x10000));
    }
}

#[test]
fn the_8821ce_and_other_vendors_are_not() {
    for (v, d) in [(0x10EC, 0xC821), (0x10EC, 0xB821), (0x10EC, 0xC822 + 1), (0x8086, 0xC822)] {
        assert!(pick(&[record(v, d)]).is_none(), "{v:#x}:{d:#x}");
    }
}

#[test]
fn a_bar_with_no_size_is_passed_over() {
    let mut r = record(0x10EC, 0xC822);
    r.bars[0].kind = BAR_KIND_MMIO;
    r.bars[0].size = 0;
    assert_eq!(pick(&[r]).map(|f| f.bar_index), Some(2));
    r.bars[2].size = 0;
    assert!(pick(&[r]).is_none());
}
