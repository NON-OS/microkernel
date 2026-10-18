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

//! Issue a PIO grant against a device's PIO BAR, as mmio::map_for_caller
//! does: the claim, the device, a PIO BAR inside the 16-bit port space, then
//! the record, while the claim still holds (K10). Access goes through
//! `access`; only the kernel executes `in` and `out`.

use super::grant::{self, PioGrant};
use super::types::{PioError, PioGrantRequest, PioGrantResult};
use super::window::{window, Refused};
use crate::hardware::broker::claim;
use crate::hardware::broker::device::BAR_KIND_PIO;
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
    let bar = device.bars[bar_idx];
    if bar.kind != BAR_KIND_PIO {
        return Err(PioError::NotPioBar);
    }
    if bar.size == 0 {
        return Err(PioError::ZeroSize);
    }
    let (port_base, port_count) = window(bar.base, bar.size).map_err(|r| match r {
        Refused::Overflow => PioError::PortOverflow,
        Refused::Reserved => PioError::ReservedPort,
    })?;

    let grant_id = grant::allocate_id();
    let record = PioGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch: claim.epoch,
        port_base,
        port_count,
    };
    claim::record_while_held(pid, req.device_id, claim.epoch, || grant::insert(record))
        .ok_or(PioError::StaleEpoch)?;
    Ok(PioGrantResult { port_base, port_count, grant_id })
}
