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

//! Steps 1 and 2 of map.rs: the caller's claim, fresh, and the BAR asked
//! for, a sized MMIO window (bar_select.rs) clear of firmware-reserved ranges.

use super::bar_select::{mappable, BarRefusal};
use super::types::{MmioMapError, MmioMapRequest};
use crate::hardware::broker::claim::{self, Claim};
use crate::hardware::broker::device::Bar;
use crate::hardware::broker::reserved;
use crate::hardware::broker::table;
use crate::hardware::broker::DeviceRecord;

pub(super) fn resolve(
    pid: u32,
    req: &MmioMapRequest,
) -> Result<(Claim, DeviceRecord, Bar), MmioMapError> {
    let claim = claim::lookup(req.device_id).ok_or(MmioMapError::NotClaimed)?;
    if claim.pid != pid {
        return Err(MmioMapError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(MmioMapError::StaleEpoch);
    }
    crate::sys::serial::println(b"[MMIO] device");
    let device = table::list()
        .into_iter()
        .find(|r| r.device_id == req.device_id)
        .ok_or(MmioMapError::UnknownDevice)?;
    let bar = mappable(&device.bars, device.bar_count, req.bar_index).map_err(|r| match r {
        BarRefusal::BadIndex => MmioMapError::BadBarIndex,
        BarRefusal::NotMmio => MmioMapError::NotMmioBar,
    })?;
    if reserved::refuses(b"MMIO", reserved::MEMORY, bar.base, bar.size, true) {
        return Err(MmioMapError::FirmwareReserved);
    }
    Ok((claim, device, bar))
}
