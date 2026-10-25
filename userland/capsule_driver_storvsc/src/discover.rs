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

//! The channel in the device table: the record the VMBus core had the
//! kernel make for the SCSI controller's channel, with its relid. The core
//! gives it once the host has offered it, so the driver waits a while for
//! it.

use nonos_libc::{mk_device_list, mk_idle_ms, Deadline, DeviceRecord, BUS_KIND_VIRT};

use crate::constants::channel::{SCSI_DEVICE, VMBUS_VENDOR};
use crate::constants::timing::CHANNEL_MS;

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Found {
    pub device_id: u64,
    pub relid: u32,
}

/// How often the table is looked at while the channel is not there.
const LOOK_MS: u64 = 100;

/// The first SCSI channel's record, looked for until CHANNEL_MS pass.
pub fn find() -> Option<Found> {
    let deadline = Deadline::after_ms(CHANNEL_MS);
    loop {
        if let Some(f) = look() {
            return Some(f);
        }
        if deadline.expired() {
            return None;
        }
        let _ = mk_idle_ms(LOOK_MS);
    }
}

fn look() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    buf[..core::cmp::min(n as usize, MAX_DEVICES)].iter().find_map(found)
}

/// The record as this driver takes it: a virtual-bus record of Microsoft's
/// with the SCSI interface. Relid 0 is never a channel's.
pub fn found(r: &DeviceRecord) -> Option<Found> {
    let ours = r.bus_kind == BUS_KIND_VIRT && r.vendor == VMBUS_VENDOR && r.device == SCSI_DEVICE;
    (ours && r.irq_source != 0).then_some(Found { device_id: r.device_id, relid: r.irq_source })
}
