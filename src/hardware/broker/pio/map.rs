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
//! in the port space clear of firmware's ports and the FADT's power registers (`power_ports`),
//! then the record while the claim holds (K10). Only the kernel runs `in`/`out` (`access`).

use super::grant::{self, PioGrant};
use super::power_ports::clear;
use super::types::{PioError, PioGrantRequest, PioGrantResult};
use super::window::{window, Refused};
use crate::hardware::broker::device::BarKind;
use crate::hardware::broker::{claim, reserved, table};

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
    if bar.kind != BarKind::Pio as u8 {
        return Err(PioError::NotPioBar);
    }
    if bar.size == 0 {
        return Err(PioError::ZeroSize);
    }
    let (port_base, port_count) = clear(window(bar.base, bar.size)).map_err(|r| match r {
        Refused::Overflow => PioError::PortOverflow,
        Refused::Reserved => PioError::ReservedPort,
    })?;
    if reserved::refuses(b"PIO", reserved::IO, bar.base, bar.size, true) {
        return Err(PioError::FirmwareReserved);
    }
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
