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

//! Claiming a host, mapping its slot, resetting it, mapping its DMA regions,
//! and bringing its card up.

use nonos_libc::mk_device_claim;

use super::super::super::disk::{EmmcDisk, DATA_BUF_BYTES};
use super::super::super::env::Log;
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::text::Line;
use super::super::discover::Found;
use super::super::dma;
use super::super::env::SerialLog;
use super::claim::Claim;
use super::command::command;
use super::embedded::check_embedded;
use super::map_slot::map_slot;
use super::reset::reset_host;
use super::say::{say_dma, say_host};
use super::sizes::DESC_TABLE_BYTES;
use super::Opened;

/// Claim `dev` with memory decoding on and its INTx pin quiet (completions
/// are polled), map slot 0 and reset the host. Only then map two DMA regions
/// (the descriptor page and the 32 KiB data buffer), below 4 GiB on a host
/// without 64-bit addressing, turn bus mastering on and bring the card up.
/// On failure the host is stopped and the claim given back (`Claim`), which
/// frees the regions, so the next attempt can claim the host again.
pub fn open(dev: &Found) -> EmmcResult<Opened> {
    let log = SerialLog;
    say_host(&log, dev);
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        log.line(Line::new().s(b"claim refused (").dec(epoch.unsigned_abs()).s(b")").as_bytes());
        return Err(EmmcError::Broker(epoch));
    }
    let epoch = epoch as u64;
    let mut claim = Claim { device_id: dev.device_id, epoch, mmio_grant: None };
    command(&log, dev.device_id, epoch, false)?;
    let io = map_slot(&log, dev, epoch, &mut claim)?;
    check_embedded(&log, dev, &io)?;
    let mut host = reset_host(io, dev)?;
    let dma64 = host.caps.dma64();
    let region =
        |len| dma::map(dev.device_id, epoch, len, dma64).inspect_err(|e| say_dma(&log, *e));
    host.table = region(DESC_TABLE_BYTES)?;
    let data = region(DATA_BUF_BYTES as u64)?;
    command(&log, dev.device_id, epoch, true)?;
    let disk = EmmcDisk::bring_up(host, data)?;
    Ok(Opened { disk, _claim: claim })
}
