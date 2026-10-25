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

//! What the shipping find_touchpad_addrs reads from a device list: only the
//! I2C-HID records, never a controller or a GPIO community on the same ACPI
//! bus; every field of each, as hid_record.rs lays it out; and a host only
//! from a kind-None bar 0, since any other kind is a window whose base the
//! list shows a non-holder as 0, which would read as PCI id 0.

use nonos_libc::{DeviceRecord, BAR_KIND_MMIO, BUS_KIND_ACPI};

use super::listed::{discovered, list, table};
use super::records::{hid, lpss, platform};

/// The broker's class for an ACPI GPIO community record (class.rs GPIO_CTRL).
const CLASS_GPIO_CTRL: u32 = 0x0080;

/// A GPIO community record: its _HID bytes in a kind-None bar 5.
fn gpio_community() -> DeviceRecord {
    let empty = DeviceRecord::empty();
    let mut r = DeviceRecord { bus_kind: BUS_KIND_ACPI, class: CLASS_GPIO_CTRL, ..empty };
    r.bars[5].base = u64::from_le_bytes(*b"INT3450\0");
    r
}

#[test]
fn only_the_i2c_hid_records_are_read_as_pads_and_every_field_is_carried() {
    let mut pad = hid(1, 2, 0x2C);
    pad.device = 0x0020;
    (pad.irq_pin, pad.irq_source) = (2, 0x17);
    pad.bars[0].size = u64::from(u32::from_le_bytes(*b"I2C1"));
    (pad.bars[0].aux, pad.bars[0].flags) = (400_000, 0x05);
    list(&table(&[lpss(0), platform(1), gpio_community()], &[pad]), None);
    let (cands, pads) = discovered();
    assert_eq!((cands.len(), pads.len()), (2, 1), "a controller or community read as a pad");
    let p = pads[0];
    assert_eq!((p.addr, p.desc_reg, p.host, p.controller_idx), (0x2C, 0x0020, Some(1), Some(1)));
    assert_eq!((p.host_name, p.speed_hz, p.info), (*b"I2C1", 400_000, 0x05));
    assert_eq!((p.gpio_pin, p.gpio_community), (0x17, 2));
}

#[test]
fn a_pad_whose_bar_0_is_a_window_names_no_host_even_with_its_base_read_as_0() {
    let mut windowed = hid(0, 0, 0x2C);
    windowed.bars[0].kind = BAR_KIND_MMIO;
    list(&table(&[lpss(0)], &[windowed, hid(0, 0, 0x2A)]), None);
    let (cands, pads) = discovered();
    assert_eq!(pads.len(), 2);
    assert!(pads[0].host.is_none() && pads[0].names_none(), "a window's 0 read as PCI id 0");
    assert!(!pads[0].names(&cands[0]));
    assert!(pads[1].names(&cands[0]), "a kind-None 0 is PCI id 0");
    assert_eq!(pads[1].host, Some(0));
}
