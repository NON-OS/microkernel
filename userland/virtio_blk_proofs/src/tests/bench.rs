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

//! The legacy device on the bench: attached to the port grant and listed
//! by the broker, as a transitional virtio-blk with its I/O window in BAR 0
//! and its INTx line routed, or with no pin at all.

use std::cell::RefCell;
use std::rc::Rc;

use nonos_libc::{attach_ports, present, Bar, DeviceRecord, BAR_KIND_PIO, BUS_KIND_PCI};

use super::legacy_dev::{Dev, LegacyBlk};

pub(super) struct Attached(pub(super) Dev);

/// A device of `qnum` entries and `capacity` sectors on the ports, its host
/// offering every feature bit, listed with `pin` (0: none) on line 11.
pub fn bench(qnum: u16, capacity: u64, pin: u8) -> Dev {
    let dev = Rc::new(RefCell::new(LegacyBlk {
        qnum,
        capacity,
        host_features: u32::MAX,
        ..LegacyBlk::default()
    }));
    attach_ports(Attached(dev.clone()));
    let mut r = DeviceRecord::empty();
    (r.device_id, r.vendor, r.device, r.bus_kind) = (9, 0x1AF4, 0x1001, BUS_KIND_PCI);
    (r.irq_pin, r.irq_line) = (pin, 11);
    r.bars[0] = Bar { kind: BAR_KIND_PIO, size: 0x40, ..r.bars[0] };
    present(&[r]);
    dev
}

/// As a dead predecessor left it: DRIVER_OK, its ring live at page 0x1234,
/// and a reset that takes `reset_reads` status reads to finish.
pub fn predecessor(dev: &Dev, reset_reads: u32) {
    let mut d = dev.borrow_mut();
    (d.status, d.pfn, d.reset_reads) = (0x0F, 0x1234, reset_reads);
}
