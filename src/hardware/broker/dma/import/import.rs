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

//! `MkDmaImportSurface`: claim -> unit in service -> attach and frames under
//! the registry's gate -> map read only -> record -> caches written out.

use super::records::{allocate_id, RECORDS};
use super::rule::{admit, admit_surface, Facts, ImportError, ImportResult};
use super::runs::{runs, PAGE};
use super::table_rule::{held_by, Import};
use crate::hardware::broker::claim;
use crate::hardware::broker::confine::{map_import, NoAddress};
use crate::kernel_core::surface_registry::pin::import_view::with_attached;

pub fn import_surface(
    pid: u32,
    device_id: u64,
    claim_epoch: u64,
    handle: u64,
) -> Result<ImportResult, ImportError> {
    // Held until the import is recorded: a release waits it out, then
    // drains what it finds.
    let held = claim::lookup(device_id);
    let claim = held.as_ref().map(|c| (c.pid == pid, c.epoch == claim_epoch));
    let iommu = crate::memory::iommu::capabilities().enforcing;
    admit(Facts { claim, iommu })?;
    let (got, runs) = with_attached(pid, handle, |attached, frames| {
        let held = held_by(&RECORDS.lock(), pid);
        admit_surface(attached, frames.len(), held)?;
        let runs = runs(frames.iter().map(|f| f.as_u64()));
        let length = (frames.len() as u64).saturating_mul(PAGE);
        let iova = match map_import(pid, device_id, &runs, length) {
            Ok(iova) => iova,
            Err(NoAddress::Unconfined) => return Err(ImportError::Unconfined),
            Err(NoAddress::Failed) => return Err(ImportError::Failed),
            Err(NoAddress::Stranded) => {
                super::drop::quarantine(&runs);
                return Err(ImportError::Failed);
            }
        };
        let id = allocate_id();
        let record = Import {
            id,
            pid,
            device_id,
            epoch: claim_epoch,
            handle,
            iova,
            length,
            runs: runs.clone(),
        };
        RECORDS.lock().push(record);
        Ok((ImportResult { device_addr: iova, length, import_id: id }, runs))
    })?;
    // A display engine's scanout need not snoop the CPU caches: what the
    // client last drew is written out before the device is told the IOVA.
    for &(phys, len) in &runs {
        super::super::cache::flush_range(phys, len);
    }
    drop(held);
    Ok(got)
}
