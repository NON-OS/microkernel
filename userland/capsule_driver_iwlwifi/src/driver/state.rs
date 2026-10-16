// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! What the driver holds for the adapter it claimed.

use crate::regs::Regs;
use crate::setup::Grants;

pub struct Driver {
    pub device_id: u64,
    pub pci_device: u16,
    pub claim_epoch: u64,
    /// Bytes of the register window the broker mapped.
    pub mmio_len: u64,
    /// The claim and the grants, until they are given back.
    pub grants: Grants,
    pub regs: Regs,
}
