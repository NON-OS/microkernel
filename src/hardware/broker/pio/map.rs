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

//! A PIO grant on a device's PIO BAR, as mmio::map_for_caller: the claim, the device, a BAR
//! in the port space clear of firmware's ports and the FADT's power registers (`bar_window`,
//! `power_ports`), then the record, overlapping no other device's grant, while the claim holds
//!. Only the kernel runs `in`/`out` (`access`).

use super::bar_window::port_window;
use super::grant::{self, PioGrant};
use super::grant_insert::insert_exclusive;
use super::types::{PioError, PioGrantRequest, PioGrantResult};
use crate::hardware::broker::claim;
use crate::hardware::broker::reserved;
use crate::hardware::broker::table;

const FLAGS_KNOWN: u32 = 0;

pub(super) fn grant_for_caller(pid: u32, req: PioGrantRequest) -> Result<PioGrantResult, PioError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(PioError::UnsupportedFlags);
    }
    let claim = claim::lookup(req.device_id).ok_or(PioError::NotClaimed)?;
    if claim.pid != pid {
        return Err(PioError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(PioError::StaleEpoch);
    }
    let device = table::list()
        .into_iter()
        .find(|r| r.device_id == req.device_id)
        .ok_or(PioError::UnknownDevice)?;
    let bar_idx = req.bar_index as usize;
    if bar_idx >= device.bars.len() || bar_idx >= device.bar_count as usize {
        return Err(PioError::BadBarIndex);
    }
    let (port_base, port_count) = port_window(device.bars[bar_idx])?;
    let bar = device.bars[bar_idx];
    if reserved::refuses(b"PIO", reserved::IO, bar.base, bar.size, true) {
        return Err(PioError::FirmwareReserved);
    }

    let grant_id = grant::allocate_id();
    insert_exclusive(PioGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch: claim.epoch,
        port_base,
        port_count,
    })?;
    Ok(PioGrantResult { port_base, port_count, grant_id })
}
