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

//! The first two steps of a map: the caller's claim, fresh, and the BAR it
//! names, a memory BAR inside the record.

use super::types::{MmioMapError, MmioMapRequest};
use crate::hardware::broker::claim;
use crate::hardware::broker::device::BAR_KIND_MMIO;
use crate::hardware::broker::table;
use crate::hardware::broker::{Bar, DeviceRecord};

pub(super) struct Claimed {
    pub(super) epoch: u64,
    pub(super) device: DeviceRecord,
    pub(super) bar: Bar,
}

pub(super) fn claimed_bar(pid: u32, req: &MmioMapRequest) -> Result<Claimed, MmioMapError> {
    let claim = claim::lookup(req.device_id).ok_or(MmioMapError::NotClaimed)?;
    if claim.pid != pid {
        return Err(MmioMapError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(MmioMapError::StaleEpoch);
    }
    let device = table::list()
        .into_iter()
        .find(|r| r.device_id == req.device_id)
        .ok_or(MmioMapError::UnknownDevice)?;
    let bar_idx = req.bar_index as usize;
    if bar_idx >= device.bars.len() || bar_idx >= device.bar_count as usize {
        return Err(MmioMapError::BadBarIndex);
    }
    let bar = device.bars[bar_idx];
    if bar.kind != BAR_KIND_MMIO {
        return Err(MmioMapError::NotMmioBar);
    }
    Ok(Claimed { epoch: claim.epoch, device, bar })
}
