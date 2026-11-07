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

//! What an import is refused for, in the order it is asked, pure:
//! kernel_proofs::surface_import.

/// Imports one holder keeps at once: the buffer on each output's plane and
/// the one replacing it while the flip lands, for up to four outputs.
pub(crate) const MAX_IMPORTS: usize = 8;

/// The device reads an imported surface and never writes it.
pub(crate) const DEVICE_READS: bool = true;
pub(crate) const DEVICE_WRITES: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportError {
    /// The caller does not hold the device's claim.
    NotClaimed,
    /// The claim was made again since the caller read its epoch.
    StaleEpoch,
    /// No IOMMU in service confines the device: nothing is lent.
    Unconfined,
    /// The caller does not hold the surface attached.
    NotAttached,
    /// The handle names no surface with frames.
    BadHandle,
    /// The holder keeps `MAX_IMPORTS` already.
    TooMany,
    /// The domain had no room or would not map the run.
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportResult {
    /// The IOVA the device reads the surface at, page 0 first.
    pub device_addr: u64,
    pub length: u64,
    pub import_id: u64,
}

/// What the import knows before it touches the domain.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Facts {
    /// The device's claim, if any: whether the caller holds it, and whether
    /// its epoch is the one named.
    pub claim: Option<(bool, bool)>,
    /// A remapping unit is translating.
    pub iommu: bool,
}

/// The device side, asked first: a surface is never looked at for a caller
/// that does not hold the device, or for a device nothing confines.
pub(crate) fn admit(f: Facts) -> Result<(), ImportError> {
    match f.claim {
        None | Some((false, _)) => Err(ImportError::NotClaimed),
        Some((true, false)) => Err(ImportError::StaleEpoch),
        Some((true, true)) if !f.iommu => Err(ImportError::Unconfined),
        Some((true, true)) => Ok(()),
    }
}

/// The surface side, under the registry's gate.
pub(crate) fn admit_surface(attached: bool, pages: usize, held: usize) -> Result<(), ImportError> {
    if !attached {
        return Err(ImportError::NotAttached);
    }
    if pages == 0 {
        return Err(ImportError::BadHandle);
    }
    if held >= MAX_IMPORTS {
        return Err(ImportError::TooMany);
    }
    Ok(())
}
