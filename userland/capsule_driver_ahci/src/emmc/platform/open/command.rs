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

//! The PCI Command word, through the broker.

use nonos_libc::{
    mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_INTX_DISABLE,
    MK_PCI_CMD_MEMORY_SPACE,
};

use super::super::super::env::Log;
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::text::Line;
use super::super::env::SerialLog;

/// Write the Command word: memory decoding on and the INTx pin quiet always,
/// bus mastering only when `bus_master`, which waits for the host reset
/// (`reset`).
///
/// The broker ORs these bits into the register today, so a word without Bus
/// Master leaves it as it stands (the boot pass sets it on every endpoint);
/// once it stores the word as written, that write turns it off. The word
/// is written whole either way, so the order holds then on its own.
pub(super) fn command(
    log: &SerialLog,
    device: u64,
    epoch: u64,
    bus_master: bool,
) -> EmmcResult<()> {
    let master = if bus_master { MK_PCI_CMD_BUS_MASTER } else { 0 };
    let word = MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_INTX_DISABLE | master;
    let r = mk_pci_config_write(device, epoch, MK_PCI_CFG_COMMAND, word);
    if r < 0 {
        log.line(
            Line::new().s(b"PCI command write refused (").dec(r.unsigned_abs()).s(b")").as_bytes(),
        );
        return Err(EmmcError::Broker(r));
    }
    Ok(())
}
