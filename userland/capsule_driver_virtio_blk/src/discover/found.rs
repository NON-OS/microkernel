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

use nonos_libc::{DeviceRecord, BUS_KIND_PCI};
use nonos_virtio::Bars;

use super::bars::{bars, first_register_bar};
use crate::constants::{VIRTIO_BLK_MODERN, VIRTIO_BLK_TRANSITIONAL, VIRTIO_VENDOR_ID};

/// The Interrupt Line value of a pin the firmware did not route.
const LINE_UNROUTED: u8 = 0xFF;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    /// The function's INTx line, when it has a pin and the firmware routed
    /// it. MSI-X is asked for first; a function with neither is refused by
    /// name at bind time, not passed over here.
    pub intx: Option<u8>,
    pub register_bar: u8,
    pub register_kind: u8,
    pub register_size: u64,
    /// Every BAR, for the transport choice and the capability checks.
    pub bars: Bars,
}

impl Found {
    /// The record as a virtio-blk function to drive, if it is one.
    pub fn of(r: &DeviceRecord) -> Option<Found> {
        let ours = r.vendor == VIRTIO_VENDOR_ID
            && r.bus_kind == BUS_KIND_PCI
            && (r.device == VIRTIO_BLK_TRANSITIONAL || r.device == VIRTIO_BLK_MODERN);
        if !ours {
            return None;
        }
        let (register_bar, register_kind, register_size) = first_register_bar(r)?;
        let routed = r.irq_pin != 0 && r.irq_line != LINE_UNROUTED;
        Some(Found {
            device_id: r.device_id,
            intx: routed.then_some(r.irq_line),
            register_bar,
            register_kind,
            register_size,
            bars: bars(r),
        })
    }
}
