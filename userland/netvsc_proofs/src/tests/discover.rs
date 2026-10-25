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

//! The channel's record in the device table: Microsoft's, on the virtual
//! bus, the network interface, its relid in `irq_source`.

use nonos_libc::{set_devices, DeviceRecord, BUS_KIND_PCI, BUS_KIND_VIRT};

use crate::discover::{find, found, Found};

fn record(bus_kind: u8, vendor: u16, device: u16, irq_source: u32) -> DeviceRecord {
    DeviceRecord { device_id: 0x99, bus_kind, vendor, device, irq_source }
}

#[test]
fn the_network_channels_record_is_taken_with_its_relid() {
    assert_eq!(found(&record(BUS_KIND_VIRT, 0x1414, 0x4E45, 14)), Some(Found { device_id: 0x99, relid: 14 }));
}

#[test]
fn every_other_record_is_passed_over() {
    for r in [
        record(BUS_KIND_VIRT, 0x1414, 0x5343, 14),
        record(BUS_KIND_VIRT, 0x1414, 0x4944, 14),
        record(BUS_KIND_VIRT, 0x1414, 0x5642, 14),
        record(BUS_KIND_VIRT, 0x15AD, 0x4E45, 14),
        record(BUS_KIND_PCI, 0x1414, 0x4E45, 14),
        record(BUS_KIND_VIRT, 0x1414, 0x4E45, 0),
    ] {
        assert_eq!(found(&r), None);
    }
}

#[test]
fn the_first_network_channel_in_the_table_is_found() {
    set_devices(vec![
        record(BUS_KIND_VIRT, 0x1414, 0x5642, 0),
        DeviceRecord { device_id: 3, ..record(BUS_KIND_VIRT, 0x1414, 0x4E45, 15) },
        DeviceRecord { device_id: 4, ..record(BUS_KIND_VIRT, 0x1414, 0x4E45, 16) },
    ]);
    assert_eq!(find(), Some(Found { device_id: 3, relid: 15 }));
}
