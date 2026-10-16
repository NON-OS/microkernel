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

//! A reader on the bus: an RTS5227 the broker lists, taken by the shipping
//! setup and brought up by the shipping chip init over the modelled part.

use nonos_libc::{set_devices, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::init::init_hw;
use crate::model::{with, Card, Knobs};
use crate::serve::medium::Medium;
use crate::serve::slot::look;
use crate::setup::{find, run, Driver};

pub const DEVICE_ID: u64 = 0x5227_0001;

pub fn rts5227() -> DeviceRecord {
    let mut r = DeviceRecord::empty();
    (r.device_id, r.bus_kind, r.pci_class) = (DEVICE_ID, BUS_KIND_PCI, 0xFF);
    (r.vendor, r.device) = (0x10EC, 0x5227);
    r.bars[0] = Bar { base: 0xF000_0000, size: 0x1000, kind: BAR_KIND_MMIO };
    r
}

/// The reader taken from the broker, its chip not yet brought up.
pub fn taken() -> Driver {
    set_devices(vec![rts5227()]);
    let found = find().expect("the reader is listed");
    run(&found).expect("the reader is taken")
}

/// The reader taken and its chip up, with `card` in the slot if any.
pub fn up(card: Option<Card>) -> Driver {
    if let Some(card) = card {
        with(|r| r.insert(card));
    }
    let mut drv = taken();
    init_hw(&mut drv).expect("the chip comes up");
    drv
}

pub fn knobs(k: Knobs) {
    with(|r| r.knobs = k);
}

/// A 512 MiB SDHC card with serial `psn`.
pub fn sdhc(psn: u32) -> Card {
    Card::sdhc(1 << 20, psn)
}

/// The reader up with `card` in its slot, looked at once: the card served.
pub fn serving(card: Card) -> (Driver, Medium) {
    let mut drv = up(Some(card));
    let mut medium = Medium::Empty;
    look(&mut drv, &mut medium);
    assert!(matches!(medium, Medium::Ready(_)), "{medium:?}");
    (drv, medium)
}
