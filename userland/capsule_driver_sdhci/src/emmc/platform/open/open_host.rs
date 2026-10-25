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
//! reset before any DMA region is mapped or bus mastering goes on.

use super::super::super::disk::DATA_BUF_BYTES;
use super::super::super::env::DmaBuf;
use super::super::super::error::EmmcResult;
use super::super::super::pci::Kind;
use super::super::super::slot::{HostParts, Slot};
use super::super::discover::Found;
use super::super::dma;
use super::super::env::{MmioWindow, SerialLog, UptimeClock};
use super::command::command;
use super::embedded::{check_embedded, check_removable};
use super::reset::reset_host;
use super::say::say_dma;
use super::sizes::DESC_TABLE_BYTES;
use super::take::take;
use super::Opened;

type Parts = HostParts<MmioWindow, UptimeClock, SerialLog>;

/// Claim `dev`, map its slot and reset the host; only then map two DMA
/// regions (the descriptor page and the 32 KiB data buffer), below 4 GiB on
/// a host without 64-bit addressing, turn bus mastering on and bring the
/// card up. On failure the host is stopped and the claim given back
/// (`Claim`), which frees the regions, so the next attempt can claim it.
pub fn open(dev: &Found) -> EmmcResult<Opened> {
    let (parts, data, claim) = ready(dev, true)?;
    Ok(Opened { slot: Slot::fixed(parts, data)?, _claim: claim })
}

/// The same for a removable slot, served with a card in it or without.
pub fn open_removable(dev: &Found) -> EmmcResult<Opened> {
    let (parts, data, claim) = ready(dev, false)?;
    Ok(Opened { slot: Slot::removable(parts, data)?, _claim: claim })
}

fn ready(dev: &Found, fixed: bool) -> EmmcResult<(Parts, DmaBuf, super::claim::Claim)> {
    let log = SerialLog;
    let (io, claim, epoch) = take(&log, dev)?;
    if fixed {
        check_embedded(&log, dev, &io)?;
    } else {
        check_removable(&log, &io)?;
    }
    let dma64 = reset_host(io, dev)?.caps.dma64();
    let region =
        |len| dma::map(dev.device_id, epoch, len, dma64).inspect_err(|e| say_dma(&log, *e));
    let table = region(DESC_TABLE_BYTES)?;
    let data = region(DATA_BUF_BYTES as u64)?;
    if !dev.acpi {
        command(&log, dev.device_id, epoch, true)?;
    }
    let parts =
        HostParts { io, clock: UptimeClock, log, table, intel_emmc: dev.kind == Kind::IntelEmmc };
    Ok((parts, data, claim))
}
