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

//! What every opened host takes: the claim, memory decoding and bus
//! mastering, and slot 0's register window.

use nonos_libc::{
    mk_device_claim, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER,
    MK_PCI_CMD_INTX_DISABLE, MK_PCI_CMD_MEMORY_SPACE,
};

use super::super::super::env::Log;
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::text::Line;
use super::super::discover::Found;
use super::super::env::{MmioWindow, SerialLog};
use super::claim::Claim;
use super::map_slot::map_slot;
use super::say::say_host;

/// Claim `dev`, turn on its memory decoding and bus mastering (and quiet
/// its INTx pin: completions are polled), and map slot 0. Returns the
/// window, the claim that gives it all back when dropped, and the epoch
/// the DMA regions are granted under.
pub(super) fn take(log: &SerialLog, dev: &Found) -> EmmcResult<(MmioWindow, Claim, u64)> {
    say_host(log, dev);
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        log.line(Line::new().s(b"claim refused (").dec(epoch.unsigned_abs()).s(b")").as_bytes());
        return Err(EmmcError::Broker(epoch));
    }
    let epoch = epoch as u64;
    let mut claim = Claim { device_id: dev.device_id, mmio_grant: None };
    let cmd = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER | MK_PCI_CMD_INTX_DISABLE;
    // An ACPI host has no command register: its decoding is the firmware's.
    let r = if dev.acpi {
        0
    } else {
        mk_pci_config_write(dev.device_id, epoch, MK_PCI_CFG_COMMAND, cmd)
    };
    if r < 0 {
        log.line(
            Line::new().s(b"PCI command write refused (").dec(r.unsigned_abs()).s(b")").as_bytes(),
        );
        return Err(EmmcError::Broker(r));
    }
    let io = map_slot(log, dev, epoch, &mut claim)?;
    Ok((io, claim, epoch))
}
