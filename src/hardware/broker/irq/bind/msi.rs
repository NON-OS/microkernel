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

//! `MkIrqBind` with `BIND_MSI`: one broker vector, delivered by the
//! function's MSI capability. Intel HDA, most AHCI controllers and many
//! Realtek parts offer MSI and no MSI-X, and without an ACPI _PRT walk their
//! INTx line from config space may not name the pin they are wired to.

use super::super::grant::{IrqGrant, IrqGrantKind, NO_LINE};
use super::super::types::{IrqBindError, IrqBindRequest, IrqBindResult};
use super::super::validate::validate_msi_request;
use super::super::{records, slots};
use super::msi_route::program_slot;
use super::quiet::msix_off;
use crate::hardware::broker::pci_index;

pub(super) fn bind_msi(
    pid: u32,
    req: IrqBindRequest,
    epoch: u64,
) -> Result<IrqBindResult, IrqBindError> {
    let handle = pci_index::lookup(req.device_id);
    let msi = handle.as_ref().and_then(|h| h.msi);
    let present = handle.as_ref().map(|_| msi.is_some());
    validate_msi_request(&req, present, records::has_message_grant(req.device_id))?;
    let handle = handle.ok_or(IrqBindError::NoDeviceHandle)?;
    let msi = msi.ok_or(IrqBindError::NoMsiCap)?;

    msix_off(&handle);
    let slot = slots::try_alloc_slot().ok_or(IrqBindError::NoVector)?;
    let (vector, irte) = match program_slot(slot, &handle.address, &msi) {
        Ok(r) => r,
        Err(e) => {
            slots::free_slot(slot);
            crate::log::info!("[IRQ] {} msi bind failed: {:?}", handle.address, e);
            return Err(e);
        }
    };
    let grant_id = records::allocate_id();
    records::insert(IrqGrant {
        grant_id,
        pid,
        device_id: req.device_id,
        claim_epoch: epoch,
        irq_source: NO_LINE,
        vector,
        flags: req.flags,
        kind: IrqGrantKind::Msi,
        device_vector: 0,
        irte,
    });
    slots::activate(slot, grant_id, NO_LINE);
    crate::log::info!("[IRQ] {} msi vector {:#x}", handle.address, vector);
    Ok(IrqBindResult { grant_id, vector })
}
