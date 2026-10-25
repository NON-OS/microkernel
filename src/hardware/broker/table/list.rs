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

extern crate alloc;

use alloc::vec::Vec;

use crate::hardware::broker::claim::lookup;
use crate::hardware::broker::shown::{shown_base, KIND_NONE};
use crate::hardware::broker::{BarKind, DeviceRecord};

use super::state::TABLE;

pub fn list() -> Vec<DeviceRecord> {
    TABLE.read().clone()
}

pub fn list_by_class(class: u32) -> Vec<DeviceRecord> {
    if class == 0 {
        return list();
    }
    TABLE.read().iter().filter(|r| r.class == class).copied().collect()
}

const _: () = assert!(BarKind::None as u8 == KIND_NONE);

/// `list_by_class` as `pid` may read it: the window bases of every record
/// it does not hold read zero.
pub fn list_shown(class: u32, pid: u32) -> Vec<DeviceRecord> {
    let mut records = list_by_class(class);
    for rec in &mut records {
        let held = lookup(rec.device_id).is_some_and(|c| c.pid == pid);
        for bar in &mut rec.bars {
            bar.base = shown_base(bar.base, bar.kind, held);
        }
    }
    records
}

pub fn contains(device_id: u64) -> bool {
    TABLE.read().iter().any(|r| r.device_id == device_id)
}

/// The device's bus kind and class.
pub(in crate::hardware::broker) fn kind_of(device_id: u64) -> Option<(u8, u32)> {
    TABLE.read().iter().find(|r| r.device_id == device_id).map(|r| (r.bus_kind, r.class))
}

pub(in crate::hardware::broker) fn flags_of(device_id: u64) -> Option<u32> {
    TABLE.read().iter().find(|r| r.device_id == device_id).map(|r| r.flags)
}

/// Whether the device's DMA snoops the CPU's caches; false for a device the
/// table does not hold, which is then treated as the safer case.
pub(in crate::hardware::broker) fn dma_coherent(device_id: u64) -> bool {
    let table = TABLE.read();
    let record = table.iter().find(|r| r.device_id == device_id);
    record.is_some_and(|r| r.flags & super::super::DEVICE_FLAG_DMA_NONCOHERENT == 0)
}
