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
//! removable one, served with or without a card. Either way the host is
//! reset before any DMA region is mapped or bus mastering goes on (`arm`).

use super::super::super::env::DmaBuf;
use super::super::super::error::EmmcResult;
use super::super::super::pci::Kind;
use super::super::super::slot::{HostParts, Slot};
use super::super::discover::Found;
use super::super::env::{MmioWindow, SerialLog, UptimeClock};
use super::embedded::{check_embedded, check_removable};
use super::reset::arm;
use super::take::take;
use super::Opened;

/// Claim `dev` with memory decoding on and its INTx pin quiet (completions
/// are polled), map slot 0 and reset the host. Only then map two DMA regions
/// (the descriptor page and the 32 KiB data buffer), below 4 GiB on a host
/// without 64-bit addressing, turn bus mastering on and bring the card up.
/// On failure the host is stopped and the claim given back (`Claim`), which
/// frees the regions, so the next attempt can claim the host again.
pub fn open(dev: &Found) -> EmmcResult<Opened> {
    let log = SerialLog;
    let (io, claim, epoch) = take(&log, dev)?;
    check_embedded(&log, dev, &io)?;
    let (host, data) = arm(&log, dev, io, epoch)?;
    let parts = parts(io, host.table, dev);
    let slot = Slot::fixed(host, parts, data)?;
    Ok(Opened { slot, _claim: claim })
}

/// The same for a removable slot, served with a card in it or without.
pub fn open_removable(dev: &Found) -> EmmcResult<Opened> {
    let log = SerialLog;
    let (io, claim, epoch) = take(&log, dev)?;
    check_removable(&log, &io)?;
    let (host, data) = arm(&log, dev, io, epoch)?;
    let parts = parts(io, host.table, dev);
    let slot = Slot::removable(host, parts, data);
    Ok(Opened { slot, _claim: claim })
}

fn parts(
    io: MmioWindow,
    table: DmaBuf,
    dev: &Found,
) -> HostParts<MmioWindow, UptimeClock, SerialLog> {
    let intel_emmc = dev.kind == Kind::IntelEmmc;
    HostParts { io, clock: UptimeClock, log: SerialLog, table, intel_emmc }
}
