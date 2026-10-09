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

//! What the MSI-X validator knows about a device, and the checks that need
//! only that.

use super::super::types::IrqBindError;

/// Snapshot of the kernel-side per-device state the validator needs.
/// Builders construct one of these from the
/// real `pci_index` entry; the validator itself only inspects the
/// fields and never goes back to the kernel.
#[derive(Clone, Copy, Debug)]
pub(in super::super) struct MsixHandleView {
    pub(in super::super) msix_present: bool,
    pub(in super::super) msix_table_size: u16,
    pub(in super::super) table_bar_in_range: bool,
    pub(in super::super) table_bar_is_mmio: bool,
    pub(in super::super) pba_bar_in_range: bool,
    pub(in super::super) pba_bar_is_mmio: bool,
}

impl MsixHandleView {
    pub(in super::super) const fn no_msix() -> Self {
        Self {
            msix_present: false,
            msix_table_size: 0,
            table_bar_in_range: false,
            table_bar_is_mmio: false,
            pba_bar_in_range: false,
            pba_bar_is_mmio: false,
        }
    }

    /// Checks 4 and 5 of the validator's order, and the table-size half of
    /// check 2, for a request of `n` vectors.
    pub(super) fn check(&self, n: usize) -> Result<(), IrqBindError> {
        if !self.msix_present {
            return Err(IrqBindError::NoMsixCap);
        }
        if (n as u16) > self.msix_table_size {
            return Err(IrqBindError::BadVectorCount);
        }
        if !self.table_bar_in_range
            || !self.table_bar_is_mmio
            || !self.pba_bar_in_range
            || !self.pba_bar_is_mmio
        {
            return Err(IrqBindError::BadMsixBar);
        }
        Ok(())
    }
}
