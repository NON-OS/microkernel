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
 * which it takes the DMA grants of the contact's pages. Registered once the
 * SynIC is up.
 *
 * A channel's record: one per channel the host offered and the core gave a
 * driver (`MkHvChannel`), vendor 0x1414 and a device by interface. Its
 * driver claims it (`claim::hold_virt` says which), takes its ring pages
 * as DMA grants on it, and signals and polls its channel through it. It
 * leaves when the host withdraws the channel or unloads the bus: whatever
 * its holder was given is revoked first, and the holder told it is gone.
 */

extern crate alloc;

use core::sync::atomic::{AtomicU64, Ordering};

use super::class::ids;
use super::device::{BusKind, DeviceRecord};
use super::table;

/// Microsoft's PCI vendor id, and "VB" for the bus.
pub const VMBUS_VENDOR: u16 = 0x1414;
pub const VMBUS_DEVICE: u16 = 0x5642;

/// A channel record's device: two letters for its interface.
pub(super) const CHANNEL_NETWORK: u16 = 0x4E45;
pub(super) const CHANNEL_SCSI: u16 = 0x5343;
pub(super) const CHANNEL_IDE: u16 = 0x4944;
pub(super) const CHANNEL_KEYBOARD: u16 = 0x4B42;
pub(super) const CHANNEL_MOUSE: u16 = 0x4D53;
pub(super) const CHANNEL_VIDEO: u16 = 0x5644;

const CHANNELS: [u16; 6] =
    [CHANNEL_NETWORK, CHANNEL_SCSI, CHANNEL_IDE, CHANNEL_KEYBOARD, CHANNEL_MOUSE, CHANNEL_VIDEO];

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

/// Register a channel's record, of a device `CHANNELS` names.
pub fn register_channel(device: u16) -> Option<u64> {
    if !CHANNELS.contains(&device) {
        return None;
    }
    let record = DeviceRecord {
        bus_kind: BusKind::Virt as u8,
        class: ids::OTHER,
        vendor: VMBUS_VENDOR,
        device,
        ..DeviceRecord::empty()
    };
    Some(table::register_platform_device(record))
}

/// Whether `device_id` is a channel's record still in the table.
pub fn is_channel(device_id: u64) -> bool {
    table::list().iter().any(|r| {
        r.device_id == device_id
            && r.bus_kind == BusKind::Virt as u8
            && r.vendor == VMBUS_VENDOR
            && CHANNELS.contains(&r.device)
    })
}

/// Whether `pid` holds the channel record `device_id`.
pub fn channel_held_by(pid: u32, device_id: u64) -> bool {
    is_channel(device_id) && super::claim::lookup(device_id).is_some_and(|c| c.pid == pid)
}

/// Whether a drained grant on `device_id` must be kept: it is a channel's
/// record and the host may still hold a GPADL over the channel's pages.
/// The first time, the core is told to close the channel.
pub(in crate::hardware::broker) fn host_may_reach(device_id: u64) -> bool {
    is_channel(device_id) && crate::arch::hv_vmbus_released(device_id)
}

/// The host tore down the channel's last GPADL: pages kept for it go.
pub fn settle_channel(device_id: u64) {
    let _ = super::dma::settle_kept_all(device_id);
}

/// A channel's record leaves: its holder's claim retired as a departed
/// device's is and its DMA grants taken back (a channel's record has no
/// window, port or interrupt), the pages kept for the channel settled (the
/// host gave it up), and the record retired, its id never reused. Its
/// driver's next call on the record finds it gone.
pub fn retire_channel(device_id: u64) {
    if !is_channel(device_id) {
        return;
    }
    if let Some(c) = super::claim::lookup(device_id) {
        if let Ok(epoch) = super::claim::retire(c.pid, device_id) {
            let _ = super::claim::quiesce_held(c.pid, device_id, epoch);
            let _ = super::dma::revoke_for_device(c.pid, device_id, epoch);
            let _ = super::claim::release(c.pid, device_id, epoch);
        }
    }
    settle_channel(device_id);
    let _ = table::remove(device_id);
}

/// Every channel's record leaves: the host unloaded the bus.
pub fn retire_all_channels() {
    let all: alloc::vec::Vec<u64> =
        table::list().iter().filter(|r| is_channel(r.device_id)).map(|r| r.device_id).collect();
    all.into_iter().for_each(retire_channel);
}
