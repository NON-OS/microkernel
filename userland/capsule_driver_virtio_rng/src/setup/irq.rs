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
//! The driver never waits on an interrupt; `fill` polls the used ring.
//! It used to bind the INTx line anyway, ack it after every request and
//! never read the device's interrupt status, so the device held the level
//! triggered line up: each ack let it fire once more and the kernel masked
//! it again with the request pending, leaving GSI 10 (shared on QEMU's q35
//! with the e1000e, xHCI, SATA and SMBus functions) masked for good with
//! its IRR set. The grant also kept any other driver from binding the
//! line. The driver now binds nothing and sets Interrupt Disable, so the
//! device drives no pin at all.

use nonos_libc::{
    mk_device_release, mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_INTX_DISABLE,
};

pub fn disable_intx(device_id: u64, claim_epoch: u64) -> Result<(), &'static str> {
    let rc =
        mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, MK_PCI_CMD_INTX_DISABLE);
    if rc >= 0 {
        return Ok(());
    }
    let _ = mk_device_release(device_id);
    Err("interrupt disable failed")
}
