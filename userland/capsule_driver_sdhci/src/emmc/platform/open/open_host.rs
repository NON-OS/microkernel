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

//! Claiming a host, mapping its slot and DMA regions, and bringing its card up.

use nonos_libc::{
    mk_device_claim, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};

use super::super::super::disk::{EmmcDisk, DATA_BUF_BYTES};
use super::super::super::env::Log;
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::pci::Kind;
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::discover::Found;
use super::super::dma::DmaRegion;
use super::super::env::{SerialLog, UptimeClock};
use super::claim::Claim;
use super::embedded::check_embedded;
use super::map_slot::map_slot;
use super::say::{say_dma, say_host};
use super::sizes::DESC_TABLE_BYTES;
use super::Opened;

/// Claim `dev`, turn on its memory decoding and bus mastering (and quiet
/// its INTx pin: completions are polled), map slot 0 and two DMA regions
/// (the descriptor page and the 32 KiB data buffer), and bring its card up.
/// Everything taken is given back on failure, so the next attempt can claim
/// the host again.
pub fn open(dev: &Found) -> EmmcResult<Opened> {
    let log = SerialLog;
    say_host(&log, dev);
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        log.line(Line::new().s(b"claim refused (").dec(epoch.unsigned_abs()).s(b")").as_bytes());
        return Err(EmmcError::Broker(epoch));
    }
    let epoch = epoch as u64;
    let mut claim = Claim { device_id: dev.device_id, mmio_grant: None };

    let cmd = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    let r = mk_pci_config_write(dev.device_id, epoch, MK_PCI_CFG_COMMAND, cmd);
    if r < 0 {
        log.line(
            Line::new().s(b"PCI command write refused (").dec(r.unsigned_abs()).s(b")").as_bytes(),
        );
        return Err(EmmcError::Broker(r));
    }
    let io = map_slot(&log, dev, epoch, &mut claim)?;

    check_embedded(&log, dev, &io)?;

    let table = DmaRegion::map(dev.device_id, epoch, DESC_TABLE_BYTES)
        .inspect_err(|e| say_dma(&log, *e))?;
    let data = DmaRegion::map(dev.device_id, epoch, DATA_BUF_BYTES as u64)
        .inspect_err(|e| say_dma(&log, *e))?;
    let host = Host::new(io, UptimeClock, SerialLog, table.buf(), dev.kind == Kind::IntelEmmc);
    let disk = EmmcDisk::bring_up(host, data.buf())?;
    Ok(Opened { disk, _data: data, _table: table, _claim: claim })
}
