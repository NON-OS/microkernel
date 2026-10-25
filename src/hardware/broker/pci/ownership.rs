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

//! Ownership + handle resolution for `MkPciConfigWrite`.

use crate::hardware::broker::claim::{self, InUse};
use crate::hardware::broker::pci_index::{self, PciHandle};

use super::types::{PciReadError, PciReadRequest, PciWriteError, PciWriteRequest};

/// The handle, and the claim held across the write (L66).
pub(super) fn resolve(
    pid: u32,
    req: &PciWriteRequest,
) -> Result<(PciHandle, InUse), PciWriteError> {
    let claim = claim::lookup(req.device_id).ok_or(PciWriteError::NotClaimed)?;
    if claim.pid != pid {
        return Err(PciWriteError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(PciWriteError::StaleEpoch);
    }
    Ok((pci_index::lookup(req.device_id).ok_or(PciWriteError::NoDeviceHandle)?, claim))
}

pub(super) fn resolve_read(
    pid: u32,
    req: &PciReadRequest,
) -> Result<(PciHandle, InUse), PciReadError> {
    let claim = claim::lookup(req.device_id).ok_or(PciReadError::NotClaimed)?;
    if claim.pid != pid {
        return Err(PciReadError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(PciReadError::StaleEpoch);
    }
    Ok((pci_index::lookup(req.device_id).ok_or(PciReadError::NoDeviceHandle)?, claim))
}
