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

use super::super::records;
use super::super::slots;
use super::super::types::{IrqBindError, IrqBindRequest, IrqBindResult};
use super::super::validate::validate_msix_request;
use super::grant_run::record_run;
use super::handle_view::handle_view;
use super::msix_route::program_routed;
use super::quiet::msi_off;
use crate::arch::interrupt::broker::{vector_of, BROKER_VEC_COUNT};
use crate::hardware::broker::pci_index;

pub(super) fn bind_msix(
    pid: u32,
    req: IrqBindRequest,
    epoch: u64,
) -> Result<IrqBindResult, IrqBindError> {
    let handle = pci_index::lookup(req.device_id);
    let view = handle.as_ref().map(handle_view);
    validate_msix_request(
        &req,
        BROKER_VEC_COUNT,
        view.as_ref(),
        records::has_message_grant(req.device_id),
    )?;
    // validate_msix_request already rejects a None handle / no-MSI-X device;
    // these guards keep a future change to the validator from turning a
    // broken invariant into a kernel panic on the trusted hardware path.
    // Resolve both before allocating vectors so no error path leaks a slot.
    let handle = handle.ok_or(IrqBindError::NoDeviceHandle)?;
    let msix = handle.msix.ok_or(IrqBindError::NoMsixCap)?;

    msi_off(&handle);
    let n = req.vector_count as usize;
    let base_slot = slots::try_alloc_contiguous(n).ok_or(IrqBindError::NoVector)?;
    let base_vector = vector_of(base_slot).ok_or(IrqBindError::NoVector)?;
    match program_routed(&handle, &msix, base_vector, n) {
        Ok(irtes) => Ok(record_run(pid, &req, epoch, base_slot, base_vector, &irtes)),
        Err(e) => {
            slots::free_contiguous(base_slot, n);
            crate::log::info!("[IRQ] {} msi-x bind failed: {:?}", handle.address, e);
            Err(e)
        }
    }
}
