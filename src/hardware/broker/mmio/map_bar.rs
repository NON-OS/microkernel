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
//! names, a memory BAR with a size inside the record (`bar_select`), clear
//! of every range firmware keeps for itself.

use super::bar_select::{mappable, BarRefusal};
use super::types::{MmioMapError, MmioMapRequest};
use crate::hardware::broker::claim;
use crate::hardware::broker::reserved;
use crate::hardware::broker::table;
use crate::hardware::broker::{Bar, DeviceRecord};

pub(super) struct Claimed {
    pub(super) epoch: u64,
    pub(super) device: DeviceRecord,
    pub(super) bar: Bar,
    /// The claim, held until the grant is recorded (L66).
    pub(super) held: claim::InUse,
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
    let bar = mappable(&device.bars, device.bar_count, req.bar_index).map_err(|r| match r {
        BarRefusal::BadIndex => MmioMapError::BadBarIndex,
        BarRefusal::NotMmio => MmioMapError::NotMmioBar,
    })?;
    if reserved::refuses(b"MMIO", reserved::MEMORY, bar.base, bar.size, true) {
        return Err(MmioMapError::FirmwareReserved);
    }
    Ok(Claimed { epoch: claim.epoch, device, bar, held: claim })
}
