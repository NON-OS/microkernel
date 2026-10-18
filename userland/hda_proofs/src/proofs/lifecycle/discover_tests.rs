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

//! The device list read into a survey: chipset controllers before a
//! graphics card's, Intel's SST engines kept out, and the DSPs that explain
//! a machine whose controllers cannot play. The list is the broker's word,
//! so a longer one than the driver reads, and records that are no HD Audio
//! controller, are held to the same bounds.

use nonos_libc::{offer_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::{CLASS_AUDIO, PCI_CLASS_MULTIMEDIA};
use crate::controller::verdict::Verdict;
use crate::discover::survey;
use crate::setup::machine_verdict;

/// A PCI audio function with a 16 KiB register BAR.
fn audio(device_id: u64, vendor: u16, device: u16, pci_subclass: u8) -> DeviceRecord {
    let mut r = DeviceRecord { device_id, vendor, device, pci_subclass, ..Default::default() };
    (r.bus_kind, r.class, r.pci_class) = (BUS_KIND_PCI, CLASS_AUDIO, PCI_CLASS_MULTIMEDIA);
    (r.bar_count, r.bars[0]) = (1, Bar { size: 0x4000, kind: BAR_KIND_MMIO });
    r
}

#[test]
fn chipset_controllers_are_tried_before_a_graphics_cards() {
    offer_devices(vec![audio(1, 0x10de, 0x10f0, 3), audio(2, 0x8086, 0xa348, 3)]);
    let s = survey();
    assert_eq!(s.controllers().map(|f| f.device_id).collect::<Vec<_>>(), [2, 1]);
    assert!(!s.is_empty() && s.bus_verdict().is_none());
}

#[test]
fn an_sst_engine_or_an_acp_names_a_machine_whose_controllers_cannot_play() {
    let mut acp = audio(4, 0x1022, 0x15e2, 0x80);
    acp.class = 0x0080;
    offer_devices(vec![audio(3, 0x8086, 0x9cb6, 1), acp]);
    let s = survey();
    assert!(s.is_empty(), "an SST engine was taken for an HD Audio controller");
    assert_eq!((s.intel_sst, s.bus_verdict()), (Some(0x9cb6), Some(Verdict::NeedsSof)));
    assert!(s.amd_acp, "the ACP was not seen");
    assert_eq!(machine_verdict(&s, None), Verdict::AmdAcp);
    assert_eq!(machine_verdict(&s, Some(Verdict::HdmiOnly)), Verdict::AmdAcp);
    assert_eq!(machine_verdict(&s, Some(Verdict::NeedsSof)), Verdict::NeedsSof);
}

#[test]
fn a_list_longer_than_the_driver_reads_and_records_no_controller_are_held() {
    let mut list: Vec<DeviceRecord> = (0..200).map(|i| audio(10 + i, 0x1af4, 0x1234, 3)).collect();
    list[0].bars[0].size = 0x100;
    list[1].bar_count = 0;
    list[2].bus_kind = 2;
    list[3].bars[0].kind = 2;
    offer_devices(list);
    let ids: Vec<u64> = survey().controllers().map(|f| f.device_id).collect();
    assert_eq!(ids, [14, 15, 16, 17], "past the four kept, or a record that is not one");
}
