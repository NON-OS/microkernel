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

//! Interrupt phase: take the device off its legacy interrupt pin.
//!
//! The driver never waits on an interrupt: transmit polls the used ring
//! and the receive ring is read when a client asks for a frame. It used
//! to bind the INTx line (or MSI-X) anyway and ack it after every
//! transmit without reading the device's interrupt status, so on INTx
//! the level-triggered line was still up at every ack, fired again and
//! was masked again, and the grant kept any other driver from binding a
//! line it shares. The driver now binds nothing and sets Interrupt
//! Disable, so the device drives no pin.

use nonos_libc::{
    mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_INTX_DISABLE,
};

use super::registers::RegisterGrant;

pub fn disable_intx(
    device_id: u64,
    claim_epoch: u64,
    reg: &RegisterGrant,
) -> Result<(), &'static str> {
    let rc =
        mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, MK_PCI_CMD_INTX_DISABLE);
    if rc >= 0 {
        return Ok(());
    }
    let reg_ok = reg.release();
    let device_ok = mk_device_release(device_id) >= 0;
    if !reg_ok || !device_ok {
        return Err("interrupt disable rollback failed");
    }
    Err("interrupt disable failed")
}
