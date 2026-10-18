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

//! The host reset every open starts from, and only after it the DMA regions
//! and bus mastering.

use super::super::super::disk::{EmmcDisk, DATA_BUF_BYTES};
use super::super::super::env::DmaBuf;
use super::super::super::error::EmmcResult;
use super::super::super::pci::Kind;
use super::super::super::sdhci::Host;
use super::super::discover::Found;
use super::super::dma;
use super::super::env::{MmioWindow, SerialLog, UptimeClock};
use super::command::command;
use super::say::say_dma;
use super::sizes::DESC_TABLE_BYTES;

pub(super) type OpenHost = Host<MmioWindow, UptimeClock, SerialLog>;

/// The host behind `io`, reset. A driver that died without its Drop (a
/// panic, a kill, an allocation failure) can leave ADMA2 running from its own
/// descriptor table, a page the kernel freed at the release, which cleared
/// bus mastering. `EmmcDisk::reset` sees Software Reset For All read 0, which
/// ends any DMA (SDHCI 3.0, 2.2.16), and refuses a host without ADMA2. Until
/// then the host has no descriptor table and no DMA region is mapped, so a
/// left-over engine has nothing of this driver's to reach.
pub(super) fn reset_host(io: MmioWindow, dev: &Found) -> EmmcResult<OpenHost> {
    let none = DmaBuf { va: 0, bus: 0, len: 0 };
    EmmcDisk::reset(Host::new(io, UptimeClock, SerialLog, none, dev.kind == Kind::IntelEmmc))
}

/// The host reset (`reset_host`), then its two DMA regions mapped, below 4 GiB
/// on a host without 64-bit addressing: the descriptor page, which becomes
/// the host's table, and the 32 KiB data buffer, returned with it. Bus
/// mastering goes on last.
pub(super) fn arm(
    log: &SerialLog,
    dev: &Found,
    io: MmioWindow,
    epoch: u64,
) -> EmmcResult<(OpenHost, DmaBuf)> {
    let mut host = reset_host(io, dev)?;
    let dma64 = host.caps.dma64();
    let region = |len| dma::map(dev.device_id, epoch, len, dma64).inspect_err(|e| say_dma(log, *e));
    host.table = region(DESC_TABLE_BYTES)?;
    let data = region(DATA_BUF_BYTES as u64)?;
    command(log, dev.device_id, epoch, true)?;
    Ok((host, data))
}
