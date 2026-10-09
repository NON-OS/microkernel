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

// x86-only: PCI MSI-X capability access + MSI-X table layout.
#![cfg(target_arch = "x86_64")]

use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::msi::{disable_msix, mask_all_msix, mask_msix_vector};
use crate::drivers::pci::types::{MsiMessage, MsixInfo, PciAddress, PciBar};

use super::super::types::IrqBindError;
use super::mmio_zero::zero_table_entry;
use super::ops::MsixOps;
use super::program::program_run;

struct RealMsixOps;

impl MsixOps for RealMsixOps {
    fn program_run(
        &self,
        address: &PciAddress,
        msix: &MsixInfo,
        bars: &[PciBar; 6],
        messages: &[MsiMessage],
    ) -> Result<(), IrqBindError> {
        program_run(address, msix, bars, messages)
    }

    fn teardown_vector(
        &self,
        _address: &PciAddress,
        msix: &MsixInfo,
        bars: &[PciBar; 6],
        device_vector: u16,
    ) {
        let _ = mask_msix_vector(msix, bars, device_vector);
        zero_table_entry(msix, bars, device_vector);
    }

    fn disable_for_device(&self, address: &PciAddress, msix: &MsixInfo) {
        let cfg = ConfigSpace::new(*address);
        let _ = mask_all_msix(&cfg, msix);
        let _ = disable_msix(&cfg, msix);
    }
}

static REAL_OPS: RealMsixOps = RealMsixOps;

pub(in crate::hardware::broker::irq) fn current_ops() -> &'static dyn MsixOps {
    &REAL_OPS
}
