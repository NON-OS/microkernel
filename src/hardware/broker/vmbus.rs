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

/*
 * The VMBus root: a record with no window and no interrupt line, which the
 * VMBus core capsule claims to be let use the SynIC's calls, and against
 * which it takes the DMA grants it hands the host (the contact's pages, a
 * channel's rings). Registered once the SynIC is up.
 */

use core::sync::atomic::{AtomicU64, Ordering};

use super::class::ids;
use super::device::{BusKind, DeviceRecord};
use super::table;

/// Microsoft's PCI vendor id, and "VB" for the bus.
pub const VMBUS_VENDOR: u16 = 0x1414;
pub const VMBUS_DEVICE: u16 = 0x5642;

const NONE: u64 = u64::MAX;
static VMBUS_ID: AtomicU64 = AtomicU64::new(NONE);

/// The record's id, registering it the first time.
pub fn register_vmbus() -> u64 {
    let id = VMBUS_ID.load(Ordering::Acquire);
    if id != NONE {
        return id;
    }
    let record = DeviceRecord {
        bus_kind: BusKind::Virt as u8,
        class: ids::OTHER,
        vendor: VMBUS_VENDOR,
        device: VMBUS_DEVICE,
        ..DeviceRecord::empty()
    };
    let id = table::register_platform_device(record);
    VMBUS_ID.store(id, Ordering::Release);
    id
}

/// Whether `pid` holds the VMBus root as `device_id`.
pub fn vmbus_held_by(pid: u32, device_id: u64) -> bool {
    device_id != NONE
        && VMBUS_ID.load(Ordering::Acquire) == device_id
        && super::claim::lookup(device_id).is_some_and(|c| c.pid == pid)
}
