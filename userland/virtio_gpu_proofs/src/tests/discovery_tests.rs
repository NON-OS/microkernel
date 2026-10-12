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

//! Which functions the driver takes. virtio-gpu is a virtio 1.0 device: the
//! specification gives it no transitional id and QEMU builds it modern, so
//! the legacy path kept for 1af4:1010 could never run, and is gone.

use nonos_libc::{DeviceRecord, BUS_KIND_PCI};

use crate::discover::match_device::is_match;

fn record(vendor: u16, device: u16, bus_kind: u8) -> DeviceRecord {
    DeviceRecord { vendor, device, bus_kind }
}

#[test]
fn only_the_modern_virtio_gpu_id_is_matched() {
    assert!(is_match(&record(0x1AF4, 0x1050, BUS_KIND_PCI)));
    for device in [0x1010, 0x1000, 0x1040, 0x1044, 0x1051, 0x0050, u16::MAX] {
        assert!(!is_match(&record(0x1AF4, device, BUS_KIND_PCI)), "{device:#x}");
    }
}

#[test]
fn another_vendor_or_bus_is_not_matched() {
    assert!(!is_match(&record(0x8086, 0x1050, BUS_KIND_PCI)));
    assert!(!is_match(&record(0x1AF4, 0x1050, BUS_KIND_PCI + 1)));
}
