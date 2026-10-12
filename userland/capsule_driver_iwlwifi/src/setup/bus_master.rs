// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Bus mastering on, once the staging DMA exists. The broker turns it off at
//! every claim (L34), and the firmware upload and every queue after it are
//! DMA.

use nonos_libc::{mk_pci_config_write, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER};

/// Only a writable bit is sent: the broker ORs it into what the register
/// holds, so the decoders stay as they are.
pub fn on(device_id: u64, claim_epoch: u64) -> Result<(), &'static str> {
    if mk_pci_config_write(device_id, claim_epoch, MK_PCI_CFG_COMMAND, MK_PCI_CMD_BUS_MASTER) < 0 {
        return Err("bus master enable failed");
    }
    Ok(())
}
