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

//! Reset, ACKNOWLEDGE and DRIVER, the features (VERSION_1, and
//! ACCESS_PLATFORM when offered; no device feature is taken: no hot plug, no
//! event reporting, no protection information), FEATURES_OK, the request
//! sizes, the request queue, DRIVER_OK. Run again to bring a device back
//! after a request it never returned.

use nonos_virtio::common::{
    accept, driver_ok, fail, setup_queue, start, CommonCfg, NotifyArea, QueueSpec,
};
use nonos_virtio::VirtioError;

use crate::constants::layout::{AVAIL_OFF, DATA_BYTES, DESC_OFF, QUEUE_MAX, QUEUE_MIN, USED_OFF};
use crate::constants::scsi::{
    CDB_SIZE, CFG_CDB_SIZE, CFG_MAX_LUN, CFG_MAX_SECTORS, CFG_MAX_TARGET, CFG_NUM_QUEUES,
    CFG_SENSE_SIZE, MAX_LUN, MAX_TARGET, REQUESTQ, SENSE_SIZE,
};

/// A 512-byte sector, the unit `max_sectors` counts in.
const SECTOR: usize = 512;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub size: u16,
    pub doorbell: usize,
    pub max_target: u16,
    pub max_lun: u16,
    pub max_bytes: usize,
}

pub fn run<R: CommonCfg>(
    common: &R,
    device: &R,
    notify: NotifyArea,
    ring: u64,
) -> Result<Limits, &'static str> {
    start(common).map_err(VirtioError::message)?;
    accept(common, 0).map_err(VirtioError::message)?;
    if device.r32(CFG_NUM_QUEUES) == 0 {
        fail(common);
        return Err("no request queue");
    }
    device.w32(CFG_CDB_SIZE, CDB_SIZE as u32);
    device.w32(CFG_SENSE_SIZE, SENSE_SIZE as u32);
    if device.r32(CFG_CDB_SIZE) != CDB_SIZE as u32
        || device.r32(CFG_SENSE_SIZE) != SENSE_SIZE as u32
    {
        fail(common);
        return Err("request sizes not taken");
    }
    let spec = QueueSpec {
        index: REQUESTQ,
        size: QUEUE_MAX,
        min_size: QUEUE_MIN,
        desc: ring + DESC_OFF as u64,
        driver: ring + AVAIL_OFF as u64,
        device: ring + USED_OFF as u64,
        vector: None,
    };
    let ready = setup_queue(common, &spec, notify).map_err(VirtioError::message)?;
    let sectors = device.r32(CFG_MAX_SECTORS) as usize;
    let max_bytes = match sectors.saturating_mul(SECTOR) {
        0 => DATA_BYTES,
        n => n.min(DATA_BYTES),
    };
    let limits = Limits {
        size: ready.size,
        doorbell: ready.notify_offset,
        max_target: device.r16(CFG_MAX_TARGET).min(MAX_TARGET),
        max_lun: device.r32(CFG_MAX_LUN).min(MAX_LUN as u32) as u16,
        max_bytes,
    };
    driver_ok(common);
    Ok(limits)
}
