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

//! What tg3_chip_reset puts back in configuration space, in its order:
//! Device Control and Status, host control, PCI state, then Command.

use crate::chip::pcicfg::{device_control, device_status, host_control, pci_state};
use crate::constants::pcicfg::{
    PCI_EXP_DEVCTL, PCI_EXP_DEVSTA, TG3PCI_MISC_HOST_CTRL, TG3PCI_PCISTATE,
};
use crate::setup::config::Config;

const PCI_COMMAND: u32 = 0x04;
/// Memory Space, Bus Master and INTx Disable: the Command bits the broker
/// lets a driver move, so the only ones put back as they were. The rest are
/// left as the reset leaves them.
const COMMAND_KEPT: u16 = 0x0406;

fn rewrite(
    cfg: &Config,
    at: u32,
    f: fn(u16) -> u16,
    what: &'static str,
) -> Result<(), &'static str> {
    let now = cfg.read(at, 2)? as u16;
    cfg.write16(at, f(now)).map_err(|_| what)
}

pub fn clear_after_reset(cfg: &Config, pcie_cap: u32) -> Result<(), &'static str> {
    rewrite(cfg, pcie_cap + PCI_EXP_DEVCTL, device_control, "device control write refused")?;
    rewrite(cfg, pcie_cap + PCI_EXP_DEVSTA, device_status, "device status write refused")
}

pub fn restore(cfg: &Config, saved_command: u16) -> Result<(), &'static str> {
    rewrite(cfg, TG3PCI_MISC_HOST_CTRL, host_control, "host control write refused")?;
    rewrite(cfg, TG3PCI_PCISTATE, pci_state, "PCI state write refused")?;
    let now = cfg.read(PCI_COMMAND, 2)? as u16;
    let command = (now & !COMMAND_KEPT) | (saved_command & COMMAND_KEPT);
    cfg.write16(PCI_COMMAND, command).map_err(|_| "PCI command restore refused")
}
