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

//! Opening a host: as a fixed slot, whose card must come up, or as a
//! removable one, served with or without a card.

use super::super::super::disk::DATA_BUF_BYTES;
use super::super::super::error::EmmcResult;
use super::super::super::pci::Kind;
use super::super::super::slot::{HostParts, Slot};
use super::super::discover::Found;
use super::super::dma::DmaRegion;
use super::super::env::{MmioWindow, SerialLog, UptimeClock};
use super::embedded::{check_embedded, check_removable};
use super::say::say_dma;
use super::sizes::DESC_TABLE_BYTES;
use super::take::take;
use super::Opened;

/// Claim `dev`, map its slot and two DMA regions (the descriptor page and
/// the 32 KiB data buffer), and bring its card up. Everything taken is
/// given back on failure, so the next attempt can claim the host again.
pub fn open(dev: &Found) -> EmmcResult<Opened> {
    let log = SerialLog;
    let (io, claim, epoch) = take(&log, dev)?;
    check_embedded(&log, dev, &io)?;
    let (table, data) = regions(&log, dev, epoch)?;
    let slot = Slot::fixed(parts(io, &table, dev), data.buf())?;
    Ok(Opened { slot, _data: data, _table: table, _claim: claim })
}

/// The same for a removable slot, served with a card in it or without.
pub fn open_removable(dev: &Found) -> EmmcResult<Opened> {
    let log = SerialLog;
    let (io, claim, epoch) = take(&log, dev)?;
    check_removable(&log, &io)?;
    let (table, data) = regions(&log, dev, epoch)?;
    let slot = Slot::removable(parts(io, &table, dev), data.buf())?;
    Ok(Opened { slot, _data: data, _table: table, _claim: claim })
}

fn regions(log: &SerialLog, dev: &Found, epoch: u64) -> EmmcResult<(DmaRegion, DmaRegion)> {
    let table =
        DmaRegion::map(dev.device_id, epoch, DESC_TABLE_BYTES).inspect_err(|e| say_dma(log, *e))?;
    let data = DmaRegion::map(dev.device_id, epoch, DATA_BUF_BYTES as u64)
        .inspect_err(|e| say_dma(log, *e))?;
    Ok((table, data))
}

fn parts(
    io: MmioWindow,
    table: &DmaRegion,
    dev: &Found,
) -> HostParts<MmioWindow, UptimeClock, SerialLog> {
    HostParts {
        io,
        clock: UptimeClock,
        log: SerialLog,
        table: table.buf(),
        intel_emmc: dev.kind == Kind::IntelEmmc,
    }
}
